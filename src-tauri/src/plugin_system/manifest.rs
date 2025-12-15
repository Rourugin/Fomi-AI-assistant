use crate::plugin_system::permissions::Permission; 
use std::path::PathBuf;
use uuid::Uuid;


pub struct PluginManifest {
    id: Uuid,
    name: String,
    version: String,
    author: String,
    description: String,
    entry_point: PathBuf,
    api_version: String,
    permissions: Vec<Permission>,
}


impl PluginManifest {
    pub fn new(name: String) -> Self {
        PluginManifest {
            id: Uuid::new_v4(),
            name,
            version: String::from("1.0.0"),
            author: String::new(),
            description: String::new(),
            permissions: Vec::new(),
            api_version: String::from("1.0"),
            entry_point: PathBuf::new(),
        }
    }

    pub fn set_author(&mut self, author: String) {
        self.author = author
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description
    }

    pub fn set_entry_point(&mut self, path: PathBuf) -> Result<(), String> {
        if path.exists() {
            self.entry_point = path;
            Ok(())
        } else {
            Err("Entry point file does not exist".to_string())
        }
    }

    pub fn set_api_version(&mut self, api_version: String) {
        self.api_version = api_version
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn entry_point(&self) -> &PathBuf {
        &self.entry_point
    }

    pub fn api_version(&self) -> &str {
        &self.api_version
    }

    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }

    pub fn add_permission(&mut self, permission: Permission) -> Result<(), String> {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
            Ok(())
        } else {
            Err("Permission already exists".to_string())
        }
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }

    pub fn remove_permission(&mut self, permission: &Permission) -> bool {
        if let Some(pos) = self.permissions.iter().position(|p|p == permission) {
            self.permissions.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Plugin name cannot be empty".to_string())
        }

        if self.version.trim().is_empty() {
            return Err("Plugin version cannot be empty".to_string())
        }

        if self.api_version.trim().is_empty() {
            return Err("API version cannot be empty".to_string())
        }

        if !self.version.chars().any(|c|c.is_digit(10)) {
            return Err("Version must contain at least one number".to_string())
        }

        Ok(())
    }

    pub fn display_info(&self) -> String {
        format!(
            "{} v{} (ID: {})\nAuthor: {}\nDescription: {}\nPermissions: {}",
            self.name,
            self.version,
            self.id,
            self.author,
            self.description,
            self.permissions.len()
        )
    }
}