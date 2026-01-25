use std::{sync::Mutex};
use crate::ai_engine::{AiCore, AiSession};


pub struct SessionManager {
    core: AiCore,
    session: Mutex<Option<AiSession>>,
    current_system_prompt: Mutex<String>,
}

impl SessionManager {
    pub fn new(core: AiCore) -> Result<SessionManager, Box<dyn std::error::Error>> {
        let system_prompt = "You are Fomi.".to_string();
        let new_session = core.start_session(&system_prompt)
            .map_err(|e| format!("Failed to create new session: {}", e))?;
        Ok(SessionManager {
            core, 
            session: Mutex::new(Option::from(new_session)), 
            current_system_prompt: Mutex::new(system_prompt),
        })
    }

    pub fn wake_up(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut guard = self.session.lock().unwrap();
        if guard.is_none() {
            let new_session = self.core.start_session("You are Fomi.")?;
            *guard = Some(new_session);
        }     
        Ok(())  
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

    pub fn think(&self, text: &str) -> Result<String, String> {
        let mut guard = self.session.lock().unwrap();
        let mut answer = "Fomi is asleep! you need to wake her up".to_string();
        if let Some(session) = guard.as_mut() {
            answer = session.infer(text).map_err(|e| format!("{}", e))?;
        }
        Ok(answer)
    }
}