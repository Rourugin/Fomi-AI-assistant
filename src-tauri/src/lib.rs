use std::{env, fs};
use tauri::Manager;
pub mod plugin_system;
mod session_manager;
pub mod ai_engine;
pub mod memory;
mod commands;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let model_path = env::current_dir()
                .expect("Failed to get current directory")
                .parent()
                .expect("Failed to get parent directory")
                .join("models/model.gguf");
            match ai_engine::AiCore::new(model_path) {
                Ok(core) => {
                    println!("AI is alive");
                    let personality_path = app.path()
                        .resolve("assets/personalities/standard.md", tauri::path::BaseDirectory::Resource)
                        .expect("Failed to resolve standard personality path");
                    let initial_prompt = fs::read_to_string(&personality_path)
                        .unwrap_or_else(|_| "You are Fomi, a helpful assistant.".to_string());
                    let app_data_dir = app.path().app_data_dir().unwrap();
                    let db_path = app_data_dir.join("memory_db");
                    let embedder_path = app_data_dir.join("models").join("all-minilm-l6-v2");

                    let memory_system_result = tauri::async_runtime::block_on(async {
                        std::fs::create_dir_all(&embedder_path).ok(); 
                        memory::MemorySystem::new(embedder_path, db_path).await
                    });

                    match memory_system_result {
                        Ok(memory) => {
                            match session_manager::SessionManager::new(core, &initial_prompt, memory) {
                                Ok(manager) => {
                                    println!("Session started");
                                    app.manage(manager);
                                }
                                Err(e) => {
                                    println!("Session error: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to init memory: {}", e);
                            panic!("Memory init failed: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Ai error: {}", e);
                }
            }

            let app_config_dir = app.path().app_config_dir().unwrap();

            if !app_config_dir.exists() {
                std::fs::create_dir_all(&app_config_dir).expect("failed to create config dir");
            };

            let manager = plugin_system::manager::PluginManager::new(app_config_dir);
            app.manage(manager);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::general::set_ignore_cursor,
            commands::general::fomi_reset,
            commands::general::fomi_think,
            commands::general::get_personalities,
            commands::general::set_personality,
            commands::plugins::get_active_plugins, 
            commands::plugins::install_plugin])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}