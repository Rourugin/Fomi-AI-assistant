use sysinfo::{Disks, System};
use std::error::Error;
use serde::Serialize;
use tauri::Manager;


#[derive(Serialize)]
pub struct SystemStatus {
    pub free_space_gb: f64,
    pub total_ram_gb: f64,
}

#[derive(Serialize)]
pub struct SystemDependencies {
    pub has_main_model: bool,
    pub has_embedder_model: bool,
    pub has_whisper: bool,
    pub has_piper: bool,
    pub has_voiceover: bool,
    pub has_voiceover_json: bool,
}

impl SystemDependencies {
    pub fn new(app: tauri::AppHandle) -> Result<SystemDependencies, Box<dyn Error>> {
        let model_dir = app
            .path()
            .app_data_dir()
            .expect("Failed to get app data directory")
            .join("models");

        let has_main_model = model_dir
            .join("model.gguf")
            .exists();

        let has_embedder_model = model_dir
            .join("embedder_model.gguf")
            .exists();

        let has_whisper = model_dir
            .join("voice")
            .join("stt")
            .join("whisper.bin")
            .exists();

        let has_piper = model_dir
            .join("voice")
            .join("tts")
            .join("piper-engine")
            .join("piper.exe")
            .exists();

        let has_voiceover = model_dir
            .join("voice")
            .join("tts")
            .join("voice.onnx")
            .exists();

        let has_voiceover_json = model_dir
            .join("voice")
            .join("tts")
            .join("voice.onnx.json")
            .exists();

        Ok(SystemDependencies {
            has_main_model,
            has_embedder_model,
            has_whisper,
            has_piper,
            has_voiceover,
            has_voiceover_json,
        })
    }

    pub fn is_all_ready(&self) -> bool {
        self.has_main_model
            && self.has_embedder_model
            && self.has_whisper
            && self.has_piper
            && self.has_voiceover
            && self.has_voiceover_json
    }
}


#[tauri::command]
pub fn get_system_info() -> SystemStatus {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let ram_gb: f64 = (sys.total_memory() as f64) / (1024f64.powi(3));

    let disks = Disks::new_with_refreshed_list();
    let mut space_gb: f64 = 0.0;
    
    if let Some(disk) = disks.list().first() {
        space_gb = (disk.available_space() as f64) / (1024f64.powi(3));
    }

    SystemStatus {
        free_space_gb: space_gb,
        total_ram_gb: ram_gb,
    }
}

#[tauri::command]
pub async fn check_setup_complete(app: tauri::AppHandle) -> SystemDependencies {
    let dependencies = SystemDependencies::new(app).expect("Error creating with system dependencies");
    return dependencies;
}