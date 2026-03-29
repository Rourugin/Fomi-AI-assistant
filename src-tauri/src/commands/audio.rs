use tokio::task::spawn_blocking;
use tauri::{State, AppHandle};
use crate::audio::stt;
use std::sync::Arc;


#[tauri::command]
pub async fn process_voice_input(state: State<'_, Arc<stt::SttEngine>>, app_handle: AppHandle, audio_bytes: Vec<u8>) -> Result<String, String> {
    let engine = state.inner().clone();

    let text = engine.recognize_via_sidecar(app_handle, &audio_bytes)
        .await
        .map_err(|e| e.to_string())?;

    Ok(text)
}
