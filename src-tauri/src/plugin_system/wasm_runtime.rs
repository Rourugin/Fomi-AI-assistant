use std::sync::Mutex;
use serde_json::Value;
use uuid::Uuid;
use wasi_common::sync::WasiCtxBuilder;
use std::path::PathBuf;
use wasmtime::{Engine, Instance, Linker, Memory, Module, Store};
use crate::plugin_system::{manifest::PluginManifest, interface::FomiTool};


pub struct WasmPlugin {
    id: Uuid,
    name: String,
    description: String,
    store: Mutex<Store<wasi_common::WasiCtx>>,
    instance: Instance,
    memory: Memory,
}

impl WasmPlugin {
    pub fn load(path: PathBuf, manifest: PluginManifest) -> Result<WasmPlugin, String> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);

        wasi_common::sync::add_to_linker(&mut linker, |s| s)
            .map_err(|e| e.to_string())
            .unwrap();
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .build();
        let mut store = Store::new(&engine, wasi);
        let module = Module::from_file(&engine, &path)
            .map_err(|e| e.to_string())
            .unwrap();
        let instance = linker.instantiate(&mut store, &module)
            .map_err(|e| e.to_string())
            .unwrap();
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or("Plugin must export 'memory'".to_string())
            .unwrap();

        Ok(WasmPlugin{
            id: Uuid::new_v4(),
            name: manifest.name().to_string(),
            description: manifest.description().to_string(),
            store: Mutex::new(store),
            instance,
            memory,
        })
    }

    fn write_string_to_memory(&self, store: &mut Store<wasi_common::WasiCtx>, text: &str) -> Result<(i32, i32), String> {
        let bytes = text.as_bytes();
        let len = bytes.len() as i32;

        let alloc_func = self.instance
            .get_typed_func::<i32, i32>(&mut *store, "fomi_alloc")
            .map_err(|_| "Plugin missing 'fomi_alloc' function".to_string())
            .unwrap();

        let ptr = alloc_func.call(&mut *store, len)
            .map_err(|e| e.to_string())
            .unwrap();

        self.memory.write(&mut *store, ptr as usize, bytes)
            .map_err(|e| e.to_string())
            .unwrap();

        Ok((ptr, len))
    }

    fn read_string_from_memory(&self, store: &mut Store<wasi_common::WasiCtx>, ptr: i32, len: i32) -> Result<String, String> {
        let mut buffer = vec![0u8; len as usize];
        self.memory.read(store, ptr as usize, &mut buffer)
            .map_err(|e| e.to_string())
            .unwrap();
        String::from_utf8(buffer)
            .map_err(|e| e.to_string())
    }
}


impl FomiTool for WasmPlugin {
    fn id(&self) -> Uuid {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({})
    }

    fn execute(&self, args: Value) -> Result<String, String> {
        let mut store_guard = self.store
            .lock()
            .map_err(|_| "Mutex poisoned")?;
        let store = &mut *store_guard;

        let json_args = args.to_string();
        let (arg_ptr, arg_len) = self.write_string_to_memory(&mut *store, &json_args).unwrap();

        let run_func = self.instance
            .get_typed_func::<(i32, i32), i32>(&mut *store, "fomi_run")
            .map_err(|_| "Plugin missing 'fomi_run'".to_string())
            .unwrap();
        let result_ptr = run_func.call(&mut *store, (arg_ptr, arg_len))
            .map_err(|e| e.to_string())
            .unwrap();

        let mut len_bytes = [0u8; 4];
        self.memory.read(&mut *store, result_ptr as usize, &mut len_bytes)
            .map_err(|e| e.to_string())
            .unwrap();
        let result_len = i32::from_le_bytes(len_bytes);

        let response_text = self.read_string_from_memory(&mut *store, result_ptr + 4, result_len)
            .unwrap();

        Ok(response_text)
    }
}