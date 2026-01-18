use crate::plugin_system::{manager::PluginManager, manifest::PluginManifest};

#[tauri::command]
pub async fn get_active_plugins(state: tauri::State<'_, PluginManager>) -> Result<Vec<PluginManifest>, String> {
    Ok(state.list_plugins())
}

#[tauri::command]
pub async fn install_plugin(state: tauri::State<'_, PluginManager>, plugin: PluginManifest) -> Result<(), ()> {
    state.add_plugin(plugin);
    Ok(())
}