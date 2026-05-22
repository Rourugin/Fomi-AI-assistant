use tauri::{State, AppHandle};
use crate::audio::{stt::SttEngine, tts::TtsEngine};
use std::sync::Arc;


#[tauri::command]
pub async fn process_voice_input(state: State<'_, Arc<SttEngine>>, app_handle: AppHandle, audio_bytes: Vec<u8>) -> Result<String, String> {
    let engine = state.inner().clone();

    let text = engine.recognize_via_sidecar(app_handle, &audio_bytes)
        .await
        .map_err(|e| e.to_string())?;

    Ok(text)
}

#[tauri::command]
pub async fn generate_audio(state: State<'_, Arc<TtsEngine>>, app_handle: AppHandle, text: String) -> Result<Vec<u8>, String> {
    let engine = state.inner().clone();
    let wav_bytes = TtsEngine::generate_audio(&engine, text)
        .await
        .map_err(|e| e.to_string())?;

    Ok(wav_bytes)
}