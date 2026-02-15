use std::{sync::Mutex};
use crate::{ai_engine::{AiCore, AiSession}, memory};


pub struct SessionManager {
    core: AiCore,
    session: Mutex<Option<AiSession>>,
    current_system_prompt: Mutex<String>,
    memory: memory::MemorySystem,
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
        })
    }

    pub fn reset(&self) -> Result<(), Box<dyn std::error::Error>> {
        let prompt_guard = self.current_system_prompt.lock().unwrap();
        let current_prompt = prompt_guard.clone();
        drop(prompt_guard);

        let new_session = self.core.start_session(&current_prompt)?;
        let mut session_guard = self.session.lock().unwrap();
        *session_guard = Some(new_session);
        Ok(())
    }

    pub fn update_personality(&self, new_prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut prompt_guard = self.current_system_prompt.lock().unwrap();
        *prompt_guard = new_prompt.to_string();
        drop(prompt_guard);

        self.reset()
    }

    pub async fn think(&self, text: &str) -> Result<String, String> {
        let system_prompt = self.current_system_prompt.lock().unwrap().clone();
        let memories = self.memory.retrieve(text, 3).await.map_err(|e| e.to_string())?;
        let context_block = if memories.is_empty() {
            String::new()
        } else {
            format!("You have the following information from your long-term memory:\n{}\n", memories.join("\n"))
        };

        let full_prompt = format!(
            "<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n{}{}<|eot_id|>\n\
            <|start_header_id|>user<|end_header_id|>\n{}<|eot_id|>\n\
            <|start_header_id|>assistant<|end_header_id|>\n",
            system_prompt, context_block, text
        );

        let session_opt = self.session.lock().unwrap().take();
        let mut session = session_opt.ok_or_else(|| "No active session".to_string())?;

        let answer = session.infer(&full_prompt).map_err(|e| format!("{}", e))?;

        *self.session.lock().unwrap() = Some(session);

        if let Err(e) = self.memory.ingest(text, "user").await {
            eprintln!("Failed to save user memory: {}", e);
        }

        if let Err(e) = self.memory.ingest(&answer, "assistant").await {
            eprintln!("Failed to save assistant memory: {}", e);
        }

        Ok(answer)
    }
}