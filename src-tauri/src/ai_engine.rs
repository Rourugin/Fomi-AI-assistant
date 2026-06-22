use llama_cpp_2::{model::{AddBos, LlamaModel, Special, params::LlamaModelParams}, token::{LlamaToken, data_array::LlamaTokenDataArray}};
use llama_cpp_2::{context::{LlamaContext, params::LlamaContextParams}, llama_backend::LlamaBackend, llama_batch::LlamaBatch};
use std::{num::NonZeroU32, path::PathBuf, sync::Arc, ffi::CStr, os::raw::{c_char, c_void}, ptr::null_mut};
use llama_cpp_sys_2::{llama_pos, ggml_log_level, llama_log_set};
use ouroboros::{self_referencing};


#[no_mangle]
pub unsafe extern "C" fn fomi_log_callback(_level: ggml_log_level, c_text: *const c_char, _user_data: *mut c_void) {
    if c_text.is_null() {
        return
    }

    if let Ok(log_str) = CStr::from_ptr(c_text).to_str() {
        if log_str.contains("ggml_cuda_graph_set_enabled") || log_str.contains("disabling CUDA graphs") {
            return
        }

        print!("{}", log_str);
    }
}


pub struct AiCore {
    model: Arc<LlamaModel>,
    _backend: Arc<LlamaBackend>,
}

impl AiCore {
    pub fn new(path: PathBuf) -> Result<AiCore, Box<dyn std::error::Error>> {
        unsafe {
            llama_log_set(Some(fomi_log_callback), null_mut());
        }

        let backend = Arc::new(LlamaBackend::init()?);
        let params = LlamaModelParams::default();
        let model = Arc::new(LlamaModel::load_from_file(&backend, path, &params)
            .map_err(|e|format!("Failed to load model: {}", e))?);
        
        Ok(AiCore {
            model,
            _backend: backend,
        })
    }

    pub fn start_session(&self, system_prompt: &str) -> Result<AiSession, Box<dyn std::error::Error>> {
        let context_params = LlamaContextParams::default();
        let prompt_tokens = self.model.str_to_token(system_prompt, AddBos::Always)?;
        AiSessionTryBuilder {
            model_handle: self.model.clone(),
            history: prompt_tokens,
            context_builder: |model_handle| {
                model_handle
                    .new_context(&self._backend, context_params.with_n_ctx(Some(NonZeroU32::new(4096).unwrap())))
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>) 
            },
        }.try_build()
    }
}


#[self_referencing]
pub struct AiSession {
    model_handle: Arc<LlamaModel>,
    history: Vec<LlamaToken>,
    #[borrows(model_handle)]
    #[covariant]
    context: LlamaContext<'this>,
}

impl AiSession {
    pub fn infer(&mut self, text: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut batch = LlamaBatch::new(2048, 1);
        self.with_mut(|fields| {
            let add_bos = if fields.history.is_empty() {
                AddBos::Always
            } else {
                AddBos::Never
            };

            let new_tokens = fields.model_handle
                .str_to_token(&text, add_bos)
                .map_err(|e| format!("Tokenize error: {}", e))?;
            batch.clear();
            let last_index = new_tokens.len().saturating_sub(1);
            for (i, token) in new_tokens.iter().enumerate() {
                let pos = fields.history.len() as i32;
                fields.history.push(*token);
                batch.add(*token, llama_pos::from(pos), &[0], i == last_index)?;
            }
            fields.context.decode(&mut batch).map_err(|e|format!("Decode prompt error: {}", e))?;

            let mut response_text = String::new();
            for _ in 0..1000 {
                let logits = fields.context.candidates_ith(batch.n_tokens() - 1).collect();
                let mut next_token_data = LlamaTokenDataArray::new(logits, false);
                let next_token = next_token_data.sample_token_greedy();

                if fields.model_handle.token_eos() == next_token {
                    break;
                }

                let token_str = fields.model_handle
                    .token_to_str(next_token, Special::Plaintext)
                    .unwrap_or(String::new());

                if fields.model_handle.token_eos() == next_token
                    || token_str.contains("<|eot_id|>")
                    || token_str.contains("<|end_of_text|>")
                    || token_str.contains("<|start_header_id|>")
                    || token_str.contains("<|end_header_id|>")
                    || token_str.contains("<|begin_of_text|>")
                {
                    break;
                }

                response_text.push_str(&token_str);
                batch.clear();
                let pos = fields.history.len() as i32;
                fields.history.push(next_token);
                batch.add(next_token, llama_pos::from(pos), &[0], true)?;
                fields.context.decode(&mut batch)
                    .map_err(|e| format!("Decode prompt error: {}", e))?;
            }

            Ok(response_text)
        })
    }

    pub fn clear_cache(& mut self) {
        self.with_mut(|fields| {
            fields.context.clear_kv_cache();
            fields.history.clear();
        });
    }
}

unsafe impl Send for AiSession {}
unsafe impl Sync for AiSession {}