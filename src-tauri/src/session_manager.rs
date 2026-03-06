use serde::Deserialize;
use std::{collections::HashMap, sync::{Arc, Mutex, RwLock}};
use crate::{ai_engine::{AiCore, AiSession}, memory, plugin_system::interface::FomiTool};


pub struct SessionManager {
    core: AiCore,
    session: Mutex<Option<AiSession>>,
    current_system_prompt: Mutex<String>,
    memory: memory::MemorySystem,
    pub registry: Arc<ToolRegistry>,
}

impl SessionManager {
    pub fn new(core: AiCore, system_prompt: &str, memory: memory::MemorySystem) -> Result<SessionManager, Box<dyn std::error::Error>> {
        let new_session = core.start_session(&system_prompt)
            .map_err(|e| format!("Failed to create new session: {}", e))?;
        Ok(SessionManager {
            core, 
            session: Mutex::new(Option::from(new_session)), 
            current_system_prompt: Mutex::new(system_prompt.to_string()),
            memory,
            registry: Arc::new(ToolRegistry::new()),
        })
    }

    pub async fn reset(&self, wipe_memory: bool) -> Result<(), Box<dyn std::error::Error>> {
        if wipe_memory {
            self.memory.wipe_memory().await?;
        }
        self.restart_session_internal(None)?;
        Ok(())
    }

    pub fn update_personality(&self, new_prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.restart_session_internal(Some(new_prompt))?;
        Ok(())
    }

    pub async fn think(&self, text: &str) -> Result<String, Box<dyn std::error::Error>> {
        let system_prompt = self.current_system_prompt.lock().unwrap().clone();
        let prompt_prefix = self.registry.generate_system_prompt_suffix();
        let memories = self.memory.retrieve(text, 3).await.map_err(|e| e.to_string())?;
        let context_block = if memories.is_empty() {
            String::new()
        } else {
            format!("You have the following information from your long-term memory:\n{}\n", memories.join("\n"))
        };

        let full_prompt = format!(
            "<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n{}{}<|eot_id|>\n\
            <|start_header_id|>user<|end_header_id|>\n{}<|eot_id|>\n\
            <|start_header_id>tools and plugins<|end_header_id|>\n{}<|eot_id|>\n\
            <|start_header_id|>assistant<|end_header_id|>\n",
            system_prompt, context_block, text, prompt_prefix
        );

        let max_steps= 5u8;
        let mut current_step = 0u8;
        let mut next_input = text;
        let role_for_next_input = "user";

        let mut prompt_part = full_prompt.clone();
        let mut final_answer = String::new();

        while current_step < max_steps {
            let session_opt = self.session.lock().unwrap().take();
            let mut session = session_opt.ok_or_else(|| "No active session".to_string()).unwrap();

            let answer = session.infer(&prompt_part).map_err(|e| format!("{}", e)).unwrap();
            if let Some(request) = parse_tool_call(&answer) {
                if let Some(plugin ) = self.registry.get(&request.tool) {
                    let tool_result = plugin.execute(request.args);

                    let result_text = match tool_result {
                        Ok(result) => result,
                        Err(e) => e,
                    };
                    prompt_part = format!("<|start_header_id|>tool_result<|end_header_id|>\n{}<|eot_id|>\n<|start_header_id|>assistant<|end_header_id|>\n", result_text);
                } else {
                    prompt_part = "<|start_header_id|>system<|end_header_id|>\nError: Tool not found. Try again.<|eot_id|>\n<|start_header_id|>assistant<|end_header_id|>\n".to_string();
                }

                *self.session.lock().unwrap() = Some(session);
                current_step += 1;

                continue;
            } else {
                final_answer = answer;
                *self.session.lock().unwrap() = Some(session);

                break;
            }
        }

        if let Err(e) = self.memory.ingest(text, "user").await {
            eprint!("Failed to save user memory: {}", e);
        }

        if let Err(e) = self.memory.ingest(&final_answer, "assistant").await {
            eprint!("Failed to save assistant memory: {}", e);
        }

        Ok(final_answer)
    }

    fn restart_session_internal(&self, new_prompt: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let new_current_prompt = {
            let mut prompt_guard = self.current_system_prompt.lock().unwrap();
            
            if let Some(new_p) = new_prompt {
                *prompt_guard = new_p.to_string();
            }
            
            prompt_guard.clone()
        };

        let new_session = self.core.start_session(&new_current_prompt)?;
        let mut session_guard = self.session.lock().unwrap();
        *session_guard = Some(new_session);

        Ok(())
    }

    pub fn register_tool(&self, tool: Box<dyn FomiTool>) {
        self.registry.register(tool);
    }
}


pub struct ToolRegistry {
    tools: RwLock<HashMap<String, Arc<dyn FomiTool>>>,
}

impl ToolRegistry {
    pub fn new() -> ToolRegistry {
        let tools_hasmap = HashMap::new();
        ToolRegistry {
            tools: RwLock::new(tools_hasmap),
        }
    }

    pub fn register(&self, tool: Box<dyn FomiTool>) {
        let mut guard = self.tools.write().unwrap();
        guard.insert(tool.name().to_string(), Arc::from(tool));
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn FomiTool>> {
        let guard =self.tools.read().unwrap();
        guard.get(name).cloned()
    }

    pub fn generate_system_prompt_suffix(&self) -> String {
        let guard = self.tools.read().unwrap();
        if guard.is_empty() {
            return String::new();
        }

        let mut prompt = String::from("\n\nAVAILABLE TOOLS:\n");
        for tool in guard.values() {
            prompt.push_str(&format!("- {}: {}\n Schema: {}\n",
                tool.name(),
                tool.description(),
                tool.parameters_schema()
            ));
        }

        prompt.push_str("\nTo use a tool, output ONLY this JSON format:\n");
        prompt.push_str("{\"tool\": \"tool_name\", \"args\": { ... }}\n");
        prompt
    }
}


#[derive(Deserialize)]
struct ToolCallRequest {
    tool: String,
    args: serde_json::Value,
}


fn parse_tool_call(text: &str) -> Option<ToolCallRequest> {
    let start = text.find("{").unwrap();
    let end = text.rfind("}").unwrap();

    if start < end {
        let slice = &text[start..=end];
        serde_json::from_str(slice).ok()
    } else {
        None
    }
}