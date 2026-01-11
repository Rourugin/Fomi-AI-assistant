use std::{sync::Mutex};
use crate::ai_engine::{AiCore, AiSession};


pub struct SessionManager {
    core: AiCore,
    session: Mutex<Option<AiSession>>,
}

impl SessionManager {
    pub fn new(core: AiCore) -> Result<SessionManager, Box<dyn std::error::Error>> {
        let new_session = core.start_session()
            .map_err(|e| format!("Failed to create new session: {}", e))?;
        Ok(SessionManager {
            core, 
            session: Mutex::new(Option::from(new_session)), 
        })
    }

    pub fn wake_up(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut guard = self.session.lock().unwrap();
        if guard.is_none() {
            let new_session = self.core.start_session()?;
            *guard = Some(new_session);
        }     
        Ok(())  
    }

    pub fn reset(&self) -> Result<(), Box<dyn std::error::Error>> {
        let new_session = self.core.start_session()?;
        let mut guard = self.session.lock().unwrap();
        *guard = Some(new_session);
        Ok(())
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