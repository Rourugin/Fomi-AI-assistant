use std::{env, error::Error, io::Write, path::PathBuf, process::{Command, Stdio}};


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

    pub async fn generate_audio(&self, text: String) -> Result<Vec<u8>, Box<dyn Error>> {
        if !self.exe_path.exists() {
            return Err(format!("Piper doesn't exist: {:?}", self.exe_path).into());
        }

        if !self.model_path.exists() {
            return Err(format!("TTS model wasn't found by path: {:?}", self.model_path).into());
        }

        let wav_path = env::temp_dir().join("fomi_output.wav");

        tokio::fs::remove_file(&wav_path).await.ok();

        let mut command = Command::new(self.exe_path.as_os_str());
        command.args([
            "--model", self.model_path.to_str().ok_or("Converting model path to str error")?, 
            "--output_file", wav_path.to_str().ok_or("Converting temp path to str error")?
        ]);
        command.stdin(Stdio::piped());
        command.stderr(Stdio::piped());

        let mut child = command.spawn().map_err(|e| format!("Failed to run Piper. Error: {}", e))?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }

        let output = child.wait_with_output()?;
        if !output.status.success() {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Piper exited with error: {}", err_msg).into());
        }

        if !wav_path.exists() {
            return Err(format!("Piper successed, but file {:?} didn't appear", wav_path).into());
        }

        let audio_data = tokio::fs::read(wav_path.clone()).await?;
        tokio::fs::remove_file(&wav_path).await.ok();

        Ok(audio_data)
    }
}