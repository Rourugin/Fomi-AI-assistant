use crate::plugin_system;

#[tauri::command]
pub async fn get_active_plugins(state: tauri::State<'_, plugin_system::manager::PluginManager>) -> Result<Vec<String>, String> {
    Ok(state.list_plugins())
}

#[tauri::command]
pub async fn install_plugin(state: tauri::State<'_, plugin_system::manager::PluginManager>, name: String) -> Result<(), ()> {
    state.add_plugin(name);
    Ok(())
}