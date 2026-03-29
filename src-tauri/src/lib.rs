use std::{env, fs, sync::Arc};
pub mod plugin_system;
mod session_manager;
use tauri::Manager;
pub mod ai_engine;
pub mod memory;
pub mod audio;
mod commands;
mod settings;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let model_path = env::current_dir()
                .expect("Failed to get current directory")
                .parent()
                .expect("Failed to get parent directory")
                .join("models")
                .join("model.gguf");
            let app_config_dir = app.path().app_config_dir()?;
            let whisper_path = model_path
                .clone()
                .parent()
                .expect("Failed to get parent directory")
                .join("voice")
                .join("stt")
                .join("whisper.bin");

            if !app_config_dir.exists() {
                std::fs::create_dir_all(&app_config_dir).expect("failed to create config dir");
            };

            let plugin_manager = plugin_system::manager::PluginManager::new(app_config_dir.clone());
            let loaded_tools = plugin_manager.load_plugins_from_disk();

            match ai_engine::AiCore::new(model_path) {
                Ok(core) => {
                    println!("AI is alive");
                    let current_settings = settings::AppSettings::load_settings(app_config_dir.clone());
                    let personality_filename = current_settings.active_personality;
                    let personality_path = app.path()
                        .resolve("assets/personalities", tauri::path::BaseDirectory::Resource)
                        .unwrap()
                        .join(personality_filename);
                    let initial_prompt = fs::read_to_string(&personality_path)?;

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
                                    for tool in loaded_tools {
                                        manager.register_tool(tool);
                                    }
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

                    match audio::stt::SttEngine::new(whisper_path) {
                        Ok(whisper) => {
                            let manager = Arc::new(whisper);
                            println!("Whisper Engine created");
                            app.manage(manager);
                        }
                        Err(e) => {
                            eprintln!("Failed to init Whisper: {}", e);
                            println!("Whisper init failed: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Ai error: {}", e);
                }
            }

            let manager = plugin_system::manager::PluginManager::new(app_config_dir);
            app.manage(manager);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::general::set_ignore_cursor,
            commands::general::fomi_reset,
            commands::general::set_fomi_avatar_state,
            commands::general::fomi_think,
            commands::general::get_personalities,
            commands::general::set_personality,
            commands::general::get_active_personality,
            commands::general::toggle_dashboard,
            commands::general::quit_app,
            commands::plugins::get_active_plugins, 
            commands::plugins::install_plugin,
            commands::audio::process_voice_input])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}