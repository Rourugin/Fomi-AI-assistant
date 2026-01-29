use std::{fmt::format, fs};
use tauri::{Manager, utils::{config::AppImageConfig, resources}};
use crate::session_manager;


#[tauri::command]
pub async fn fomi_reset(state: tauri::State<'_, session_manager::SessionManager>) -> Result<(), String> {
    state.reset().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fomi_think(state: tauri::State<'_, session_manager::SessionManager>, text: String) -> Result<String, String> {
    state.think(&text).map_err(|e| e.to_string())
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
pub async fn set_personality(name: String, state: tauri::State<'_, session_manager::SessionManager>, app: tauri::AppHandle) -> Result<(), String> {
    let filename = format!("{}.md", name);
    let resource_dir = app.path().resolve("assets/personalities", tauri::path::BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;
    let file_path = resource_dir.join(filename);
    let prompt_text = fs::read_to_string(file_path).map_err(|e| e.to_string())?;

    state.update_personality(&prompt_text).map_err(|e| e.to_string())?;
    Ok(())
}