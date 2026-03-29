use std::{env, error::Error, os::raw, path::PathBuf};
use tauri_plugin_shell::ShellExt;
use tauri::AppHandle;


pub struct SttEngine {
    model_path: PathBuf,
}

impl SttEngine {
    pub fn new(path: PathBuf) -> Result<SttEngine, Box<dyn Error>> {
        Ok(SttEngine {
            model_path: path
        })
    }

    pub async fn recognize_via_sidecar(&self, app_handle: AppHandle, wav_bytes: &[u8]) -> Result<String, Box<dyn Error>> {
        let temp_dir = env::temp_dir();
        let file_path = temp_dir.join("fomi_input_audio.wav");

        tokio::fs::write(&file_path, wav_bytes).await?;

        let command = app_handle
            .shell()
            .sidecar("whisper-whisper-cli")?
            .args(["-m", self.model_path.to_str().ok_or("Converting model path to str error")?])
            .args(["-f", file_path.to_str().ok_or("Converting file path to str error")?])
            .args(["-nt", "1"])
            .args(["-nc"]);

        let result = command.output().await?;

        let raw_text = String::from_utf8(result.stdout)?;

        tokio::fs::remove_file(&file_path).await?;

        Ok(raw_text.trim().to_string())
    }
}