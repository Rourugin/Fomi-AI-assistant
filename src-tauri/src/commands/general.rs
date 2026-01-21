use crate::session_manager;

#[tauri::command]
pub async fn fomi_wake_up(state: tauri::State<'_, session_manager::SessionManager>) -> Result<(), String> {
    state.wake_up().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fomi_reset(state: tauri::State<'_, session_manager::SessionManager>) -> Result<(), String> {
    state.reset().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fomi_think(state: tauri::State<'_, session_manager::SessionManager>, text: String) -> Result<String, String> {
    state.think(&text).map_err(|e| e.to_string())
}