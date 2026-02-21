use serde::{Serialize, Deserialize};
use std::{fs, path::PathBuf};


#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub active_personality: String,
}

impl AppSettings {
    pub fn load_settings(config_dir: PathBuf) -> AppSettings {
        let path = config_dir.join("fomi_settings.json");

        if path.exists() {
            let data = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or(AppSettings { 
                active_personality: "standard.md".to_string(),
            })
        }
        else {
            AppSettings { 
                active_personality: "standard.md".to_string(),
            }
        }
    }

    pub fn save_personality_choice(config_dir: PathBuf, filename: String) -> Result<(), Box<dyn std::error::Error>> {
        let path = config_dir.join("fomi_settings.json");
        let new_app_settings = AppSettings {
            active_personality: filename,
        };

        let json_string = serde_json::to_string_pretty(&new_app_settings)?;
        fs::write(path, json_string)?;

        Ok(())
    }
}