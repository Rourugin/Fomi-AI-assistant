use std::fs;
use tauri::Manager;
use crate::{session_manager, settings};


#[tauri::command]
pub async fn set_ignore_cursor(window: tauri::Window, ignore: bool) -> Result<(), String> {
  window.set_ignore_cursor_events(ignore).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fomi_reset(state: tauri::State<'_, session_manager::SessionManager>, wipe: bool) -> Result<(), String> {
    state.reset(wipe).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fomi_think(state: tauri::State<'_, session_manager::SessionManager>, text: String) -> Result<String, String> {
    state.think(&text).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_personalities(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let resource_path = app.path().resolve("assets/personalities", tauri::path::BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

    let mut names = Vec::new();
    if let Ok(entries) = fs::read_dir(resource_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        names.push(stem.to_string());
                    }
                }
            }
        }
    }

    Ok(names)
}

#[tauri::command]
pub async fn set_personality(state: tauri::State<'_, session_manager::SessionManager>, app: tauri::AppHandle, name: String, wipe: bool) -> Result<(), String> {
    let filename = format!("{}.md", name);
    let resource_dir = app.path().resolve("assets/personalities", tauri::path::BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;
    let file_path = resource_dir.join(filename.clone());
    let prompt_text = fs::read_to_string(file_path).map_err(|e| e.to_string())?;

    state.update_personality(&prompt_text).map_err(|e| e.to_string())?;
    if wipe {
        state.reset(wipe).await.map_err(|e| e.to_string())?;
    }

    let app_config_dir = app.path().app_config_dir().unwrap();
    settings::AppSettings::save_personality_choice(app_config_dir, filename)
        .map_err(|e| format!("Failed to save personality choice: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_active_personality(app: tauri::AppHandle) -> Result<String, String> {
    let config_dir = app.path().app_config_dir().unwrap();
    let settings = settings::AppSettings::load_settings(config_dir);

    let filename = settings.active_personality;
    let pretty_filename = filename.replace(".md", "");
    Ok(pretty_filename)
}

#[tauri::command]
pub async fn quit_app(app: tauri::AppHandle) -> Result<(), String> {
    app.exit(0);
    Ok(())
}