use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use serde_json::Value;
use std::hash::Hash;
use uuid::Uuid;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum FileAccessLevel {
    ReadOnly,
    ReadWrite,
    Execute,
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum NetworkAccessLevel {
    LocalOnly,
    SpecificDomains,
    FullInternet,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum SystemControl {
    ProcessManagement,
    WindowControl,
    PowerManagement,
    HardwareInfo,
    ClipboardAccess,
    NotificationSend,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum PermissionStatus {
    Granted,
    Denied,
    NotDecided,
    SystemDenied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Paranoid,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CheckResult{
    Granted,
    Denied(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Permission {
    FileSystem {
        path: PathBuf,
        access_level: FileAccessLevel,
    },

    Network {
        access_level: NetworkAccessLevel,
        domains: Vec<String>,
    },

    System {
        control: SystemControl,
        params: Value,
    },

    Audio {
        microphone_access: bool,
        speaker_access: bool,
    },

    Video {
        camera_access: bool,
        screen_recording_access: bool,
    },

    Custom {
        id: String,
        params: Value,
    },
}

impl std::hash::Hash for Permission {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Permission::FileSystem {path, access_level} => {
                0.hash(state);
                path.hash(state);
                access_level.hash(state);
            }
            Permission::Network {access_level, domains} => {
                1.hash(state);
                access_level.hash(state);
                domains.hash(state);
            }
            Permission::System {control, ..} => {
                2.hash(state);
                control.hash(state);
            }
            Permission::Audio {microphone_access, speaker_access} => {
                3.hash(state);
                microphone_access.hash(state);
                speaker_access.hash(state);
            }
            Permission::Video {camera_access, screen_recording_access} => {
                4.hash(state);
                camera_access.hash(state);
                screen_recording_access.hash(state);
            }
            Permission::Custom {id, ..} => {
                5.hash(state);
                id.hash(state);
            }
        }
    }
}


struct PluginPolicy {
    plugin_id: Uuid,
    permissions: HashMap<Permission, PermissionStatus>,
}

impl PluginPolicy {
    fn new(plugin_id: Uuid)-> Self {
        PluginPolicy {
            plugin_id,
            permissions: HashMap::new(),
        }
    }

    fn allowed(&self, permission: &Permission) -> bool {
        match self.permissions.get(permission) {
            Some(PermissionStatus::Granted)=>true,
            _=>false,
        }
    }

    fn update_status(&mut self, permission: Permission, status: PermissionStatus) {
        self.permissions.insert(permission, status);
    }

    fn get_status(&self, permission: &Permission) -> Option<&PermissionStatus> {
        self.permissions.get(permission)
    }

    fn permission_iter(&self) -> impl Iterator<Item = (&Permission, &PermissionStatus)> {
        self.permissions.iter()
    }
}

struct SystemConstraints {
    denied_paths: Vec<String>,
    denied_domains: Vec<String>,
    available_devices: HashMap<String, bool>,
    security_level: SecurityLevel,
}

impl SystemConstraints {
    fn new(security_level: SecurityLevel) -> Self {
        SystemConstraints {
            denied_paths: Vec::new(),
            denied_domains: Vec::new(),
            available_devices: HashMap::new(),
            security_level,
        }
    }

    fn add_denied_path(&mut self, path: String) {
        self.denied_paths.push(path);
    }
}

pub struct Logger;

impl Logger {
    fn log(&self, _message: &str) {
        println!("[LOG] {}", _message);
    }
}

#[derive(Debug, Clone)]
pub struct CheckContext {
    pub is_screen_locked: bool,
    pub battery_level: Option<u8>,
    pub network_available: bool,
}

impl Default for CheckContext {
    fn default() -> Self {
        CheckContext {
            is_screen_locked: false,
            battery_level: Some(100),
            network_available: true,
        }
    }
}

pub struct PermissionChecker {
    plugin_policies: HashMap<Uuid, PluginPolicy>,
    system_constraints: SystemConstraints,
    logger: Logger,
}

impl PermissionChecker {
    pub fn new(security_level: SecurityLevel) -> Self {
        PermissionChecker {
            plugin_policies: HashMap::new(),
            system_constraints: SystemConstraints::new(security_level),
            logger: Logger,
        }
    }

    pub fn register_plugin(&mut self, plugin_id: Uuid, permissions: Vec<Permission>) -> Result<(), String> {
        let mut policy = PluginPolicy::new(plugin_id);

        for permission in permissions {
            let status = if self.is_dangerous_permission(&permission) {
                PermissionStatus::NotDecided
            } else {
                PermissionStatus::Granted
            };

            policy.update_status(permission, status);
        }

        self.plugin_policies.insert(plugin_id, policy);
        self.logger.log(&format!("Plugin {} registered", plugin_id));
        Ok(())
    }

    fn is_dangerous_permission(&self, permission: &Permission) -> bool {
        match permission {
            Permission::FileSystem {path, access_level} => {
                let path_str = path.to_string_lossy().to_string();
                self.system_constraints.denied_paths.iter().any(|denied|path_str.contains(denied))
            }
            Permission::Network {access_level, ..} => {
                matches!(access_level, NetworkAccessLevel::FullInternet)
            }
            Permission::System {control, ..} => {
                matches!(control, SystemControl::PowerManagement)
            }
            _=>false,
        }
    }

    pub fn check(&self, plugin_id: &Uuid, permission: &Permission, context: Option<&CheckContext>) -> CheckResult {
        let policy = match self.plugin_policies.get(plugin_id) {
            Some(p) => p,
            None => {
                self.logger.log(&format!("Plugin {:?} not found", plugin_id));
                return CheckResult::Denied("Plugin not found".to_string());
            }
        };

        if !policy.allowed(permission) {
            self.logger.log(&format!("Plugin denied by policy {:?}", permission));
            return CheckResult::Denied("Permission denied by policy".to_string());
        }

        if !self.passes_system_constraints(permission) {
            self.logger.log(&format!("Permission denied by system {:?}", permission));
            return CheckResult::Denied("Permission denied by system".to_string());
        }

        if let Some(ctx) = context {
            match self.check_context(permission, ctx) {
                Ok(_) => (),
                Err(reason) => {
                    self.logger.log(&format!("Permission denied by context: {:?}", reason));
                    return CheckResult::Denied("Permission denied by context".to_string());
                }
            }
        }

        CheckResult::Granted
    }

    fn passes_system_constraints(&self, permission: &Permission) -> bool {
        match permission {
            Permission::FileSystem {path, ..} => {
                let path_str = path.to_string_lossy().to_string();
                !self.system_constraints.denied_paths.iter().any(|denied|path_str.contains(denied))
            }

            Permission::Network {domains, ..} => {
                domains.iter().all(|domain|{
                    !self.system_constraints.denied_domains.iter().any(|denied|domain.contains(denied))
                })
            }
            _=>true,
        }
    }

    fn check_context(&self, permission: &Permission, context: &CheckContext) -> Result<(), String> {
        match permission {
            Permission::FileSystem {path, ..} => {
                if !path.exists() {
                    return Err("File does not exist".to_string());
                }
                Ok(())
            }

            Permission::Network {..} => {
                if !context.network_available {
                    return Err("Network is unavailable".to_string());
                }
                Ok(())
            }

            Permission::System {control, ..} => {
                match control {
                    SystemControl::PowerManagement => {
                        if let Some(battery) = context.battery_level {
                            if battery < 10 {
                                return Err("Battery too low (<10%)".to_string());
                            }
                        }
                        Ok(())
                    }
                    SystemControl::ClipboardAccess if context.is_screen_locked => {
                        return Err("Cannot access clipboard when screen is locked".to_string());
                    }
                    _=>Ok(())
                }
            }

            _=>Ok(())
        }
    }

    pub(crate) fn request_user_approval(&self, plugin_id: &Uuid, permission: &Permission) -> PermissionStatus {
        PermissionStatus::NotDecided
    }

    pub fn update_user_decision(&mut self, plugin_id: &Uuid, permission: &Permission, decision: bool) -> Result<(), String> {
        let policy = match self.plugin_policies.get_mut(plugin_id) {
            Some(p) => p,
            None => return Err(format!("Plugin {:?} not found", plugin_id)),
        };

        if policy.get_status(permission).is_none() {
            return Err("Plugin did not declare this permission".to_string());
        }

        let status = if decision {
            PermissionStatus::Granted
        } else {
            PermissionStatus::Denied
        };

        policy.update_status(permission.clone(), status);
        self.logger.log(&format!("User {} permission {:?} for plugin {:?}",
            if decision {"granted"} else {"denied"},
            permission,
            plugin_id,
        ));

        Ok(())
    }

    pub fn get_pending_requests(&self) -> Vec<(Uuid, Permission)> {
        let mut results = Vec::new();

        for (plugin_id, policy) in &self.plugin_policies {
            for (permission, status) in policy.permissions {
                if PermissionStatus::NotDecided == status {
                    results.push((plugin_id.clone(), permission.clone()))
                }
            }
        }

        results
    }
}
