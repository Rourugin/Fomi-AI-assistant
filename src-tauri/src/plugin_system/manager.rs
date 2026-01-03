use std::{fs::{self, OpenOptions}, sync::Mutex};
use serde_json::{from_reader, to_writer_pretty};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;


pub struct PluginManager {
    names: Mutex<Vec<String>>,
    config_file_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct PluginData {
    plugins: Vec<String>,
}

impl PluginManager {
    pub fn new(config_dir: PathBuf) -> Self {
        let plugin_file_path = config_dir.join("plugins.json");
        let initial_plugins = Self::load_from_file(plugin_file_path.clone()).unwrap_or_else(|_| {
            Vec::new()
        });

        PluginManager {
            names: Mutex::new(initial_plugins),
            config_file_path: plugin_file_path,
        }
    }

    pub fn list_plugins(&self) -> Vec<String> {
        let guard = self.names.lock().unwrap();
        return guard.clone();
    }

    pub fn add_plugin(&self, name: String) {
        {
            let mut guard = self.names.lock().unwrap();
            guard.push(name);
        }

        if let Err(e) = self.save_to_file() {
            eprintln!("Failed to save plugins: {}", e);
        }
    }

    fn save_to_file(&self) -> Result<(), std::io::Error> {
        let guard = self.names.lock().unwrap();
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

    fn load_from_file(path: PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let file = fs::File::open(path)?;
        let data: PluginData = from_reader(file)?;
        Ok(data.plugins)
    }
}