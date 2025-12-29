use crate::plugin_system::permissions::{PermissionChecker, SecurityLevel, Logger};
use crate::plugin_system::{manifest::PluginManifest, wasm_runtime::WasmRuntime};
use std::collections::HashMap;
use uuid::Uuid;


enum PluginState {
    Loading,
    Active,
    Error,
}

struct PluginInstance {
    id: Uuid,
    manifest: PluginManifest,
    state: PluginState,
}

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

    fn load_plugin(path: PathBuf) {

    }

    fn unload_plugin() {

    }

    fn call_plugin() {

    }

    fn list_plugins() {

    }
}