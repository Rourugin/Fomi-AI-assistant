use crate::plugin_system::manifest::PluginManifest;
use std::{fs::{self, OpenOptions}, sync::Mutex};
use serde_json::{from_reader, to_writer_pretty};
use serde::{Deserialize, Serialize};
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
}