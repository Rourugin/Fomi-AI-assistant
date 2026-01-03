use tauri::Manager;

pub mod plugin_system;
mod commands;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_config_dir = app.path().app_config_dir().unwrap();

            if !app_config_dir.exists() {
                std::fs::create_dir_all(&app_config_dir).expect("failed to create config dir");
            };

            let manager = plugin_system::manager::PluginManager::new(app_config_dir);
            app.manage(manager);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::general::say_bye, 
            commands::general::greet, 
            commands::plugins::get_active_plugins, 
            commands::plugins::install_plugin])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}