use tokio::task::spawn_blocking;
use crate::audio::stt;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn process_voice_input(state: State<'_, Arc<stt::SttEngine>>, audio_bytes: Vec<u8>) -> Result<String, String> {
    let engine = state.inner().clone();

    let result = spawn_blocking(move || {
        engine.transcribe_wav(&audio_bytes)
    })
    .await
    .map_err(|e| e.to_string())?;

    result.map_err(|e| e.to_string())
}
