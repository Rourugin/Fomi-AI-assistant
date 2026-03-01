use crate::plugin_system::{interface::FomiTool, manifest::PluginManifest, wasm_runtime::WasmPlugin};
use std::{fs::{self, OpenOptions}, sync::Mutex};
use serde_json::{from_reader, to_writer_pretty};
use serde::{Deserialize, Serialize};
use tauri::utils::acl::manifest;
use std::path::PathBuf;


pub struct PluginManager {
    plugins: Mutex<Vec<PluginManifest>>,
    config_file_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct PluginData {
    plugins: Vec<PluginManifest>,
}

impl PluginManager {
    pub fn new(config_dir: PathBuf) -> Self {
        let plugin_file_path = config_dir.join("plugins.json");
        let initial_plugins: Vec<PluginManifest> = Self::load_from_file(plugin_file_path.clone()).unwrap_or_else(|_| {
            Vec::new()
        });

        PluginManager {
            plugins: Mutex::new(initial_plugins),
            config_file_path: plugin_file_path,
        }
    }

    pub fn list_plugins(&self) -> Vec<PluginManifest> {
        let guard = self.plugins.lock().unwrap();
        return guard.clone();
    }

    pub fn add_plugin(&self, plugins: PluginManifest) {
        {
            let mut guard = self.plugins.lock().unwrap();
            guard.push(plugins);
        }

        if let Err(e) = self.save_to_file() {
            eprintln!("Failed to save plugins: {}", e);
        }
    }

    fn save_to_file(&self) -> Result<(), std::io::Error> {
        let guard = self.plugins.lock().unwrap();
        let data = PluginData {
            plugins: guard.clone(),
        };
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.config_file_path)?;

        to_writer_pretty(file, &data)?;
        Ok(())
    }

    fn load_from_file(path: PathBuf) -> Result<Vec<PluginManifest>, Box<dyn std::error::Error>> {
        let file = fs::File::open(path)?;
        let data: PluginData = from_reader(file)?;
        Ok(data.plugins)
    }

    pub fn load_plugins_from_disk(&self) -> Vec<Box<dyn FomiTool>> {
        let mut loaded_tools = Vec::new();

        let plugins_dir = self.config_file_path.parent().unwrap().join("plugins");
        if !plugins_dir.exists() {
            fs::create_dir_all(&plugins_dir).unwrap_or_default();
            return loaded_tools
        }

        if let Ok(entries) = fs::read_dir(plugins_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let manifest_path = path.join("manifest.json");
                    let wasm_path = path.join("plugin.wasm");

                    if manifest_path.exists() && wasm_path.exists() {
                        if let Ok(file) = fs::File::open(&manifest_path){
                            if let Ok(manifest) = serde_json::from_reader::<_, PluginManifest>(file) {
                                match WasmPlugin::load(wasm_path, manifest) {
                                    Ok(plugin) => {
                                        println!("Loaded plugin: {}", plugin.name());
                                        loaded_tools.push(Box::new(plugin));
                                    }
                                    Err(e) => {
                                        eprint!("Failed to load WASM for {}: {}", path.display(), e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        loaded_tools
    }
}