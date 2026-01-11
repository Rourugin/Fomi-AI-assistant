use crate::session_manager;

#[tauri::command]
pub async fn greet(name: String) -> String {
    format!("Hello, {}! You've been greeted from Fomi! <3", name)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn say_bye(input: String) -> String {
    if input.to_lowercase().contains("hello") {
        return format!("Bye!");
    } else {
        return format!("");
    };
}

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
    state.think(&text)
}