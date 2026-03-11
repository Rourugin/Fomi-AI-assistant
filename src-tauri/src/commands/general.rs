use std::fs;
use tauri::{Emitter, Manager};
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
pub async fn set_fomi_avatar_think(app: tauri::AppHandle) -> Result<String, String> {
    let state = "think".to_string();

    if let Some(main_window) = app.get_webview_window("main") {
        let _ = main_window.emit("avatar-state-change", state.clone());
    }

    Ok(state)
}

#[tauri::command]
pub async fn fomi_think(app: tauri::AppHandle, state: tauri::State<'_, session_manager::SessionManager>, text: String) -> Result<String, String> {
    let response = state.think(&text).await.map_err(|e| e.to_string())?;

    if let Some(main_window) = app.get_webview_window("main") {
        let _ = main_window.emit("show-subtitle", &response);
    }

    Ok(response)
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
pub async fn toggle_dashboard(app: tauri::AppHandle) -> Result<(), String> {
    let dashboard_window = app.get_webview_window("dashboard");
    let main_window = app.get_webview_window("main");

    if let Some(window) = dashboard_window {
        if window.is_visible().unwrap() {
            window.hide().unwrap();
            window.set_ignore_cursor_events(true).map_err(|e| e.to_string())?;
            if let Some(main_w) = main_window {
                main_w.set_always_on_top(true).map_err(|e| e.to_string())?;
            }
        } else {
            if let Some(main_w) = main_window {
                main_w.set_always_on_top(false).map_err(|e| e.to_string())?;
            }
            window.show().unwrap();
            window.set_focus().unwrap();
            window.set_ignore_cursor_events(false).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn quit_app(app: tauri::AppHandle) -> Result<(), String> {
    app.exit(0);
    Ok(())
}