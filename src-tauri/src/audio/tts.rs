use std::{env, error::Error, path::PathBuf};
use tauri_plugin_shell::{ShellExt, process::{CommandEvent, TerminatedPayload}};
use tauri::AppHandle;

use crate::audio;


pub struct TtsEngine {
    model_path: PathBuf,
}


impl TtsEngine {
    pub fn new(path: PathBuf) -> Result<TtsEngine, Box<dyn Error>> {
        Ok(TtsEngine {
            model_path: path
        })
    }

    pub async fn generate_audio(&self, app_handle: AppHandle, text: String) -> Result<Vec<u8>, Box<dyn Error>> {
        let wav_path = env::temp_dir().join("fomi_output.wav");

        let command = app_handle
            .shell()
            .sidecar("piper")?
            .args(["--model", self.model_path.to_str().ok_or("Converting model path to str error")?])
            .args(["--output_file", wav_path.to_str().ok_or("Converting temp path to str error")?]);

        let (mut rx, mut child) = command.spawn()?;
        child.write(format!("{}\n", text).as_bytes())?;

        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Terminated(payload) => {
                    println!("Piper exited with code: {:?}", payload.code);
                    break;
                },
                CommandEvent::Error(err) => {
                    eprintln!("Error Piper: {:?}", err);
                },
                CommandEvent::Stdout(line) => {
                    println!("Piper writes: {:?}", String::from_utf8_lossy(&line));
                },
                CommandEvent::Stderr(line) => {
                    println!("Piper log: {:?}", String::from_utf8_lossy(&line));
                }
                _ => {},
            }
        }

        let audio_data = tokio::fs::read(wav_path.clone()).await?;
        tokio::fs::remove_file(&wav_path).await.ok();
        Ok(audio_data)
    }
}