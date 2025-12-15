use crate::plugin_system::{manifest::PluginManifest, permissions::PermissionChecker};
use std::collections::HashMap;
use uuid::Uuid;

struct PluginManager {
    permission_checker: PermissionChecker,
    manifest: HashMap<Uuid, PluginManifest>,
    loaded_plugins: HashMap<Uuid, PluginInstance>,
    wasm_runtime: Option<WasmRuntime>,
    logger: Logger,
}


impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            permission_checker: PermissionChecker::new(SecurityLevel::Medium),
            manifest: HashMap::new(),
            loaded_plugins: HashMap::new(),
            wasm_runtime: None,
            logger: Logger,
        }
    }

    
}