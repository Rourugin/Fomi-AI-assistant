use std::{env, error::Error, io::Write, path::PathBuf, process::{Command, Stdio}};
use tauri_plugin_shell::{ShellExt, process::CommandEvent};
use tauri::AppHandle;


pub struct TtsEngine {
    model_path: PathBuf,
    exe_path: PathBuf,
}


impl TtsEngine {
    pub fn new(model_path: PathBuf, exe_path: PathBuf) -> Result<TtsEngine, Box<dyn Error>> {
        Ok(TtsEngine {
            model_path,
            exe_path
        })
    }

    pub async fn generate_audio(&self, app_handle: AppHandle, text: String) -> Result<Vec<u8>, Box<dyn Error>> {
        let wav_path = env::temp_dir().join("fomi_output.wav");

        let mut command = Command::new(self.exe_path.as_os_str());
        command.args([
            "--model", self.model_path.to_str().ok_or("Converting model path to str error")?, 
            "--output_file", wav_path.to_str().ok_or("Converting temp path to str error")?
        ]);
        command.stdin(Stdio::piped());

        let mut child = command.spawn()?;
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(text.as_bytes())?;
        drop(stdin);
        child.wait()?;

        let audio_data = tokio::fs::read(wav_path.clone()).await?;
        tokio::fs::remove_file(&wav_path).await.ok();
        Ok(audio_data)
    }
}