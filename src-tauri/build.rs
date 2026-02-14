use std::env;
use std::fs;
use std::path::Path;

fn main() {
    tauri_build::build();

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "windows" {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let dll_path = Path::new(&manifest_dir).join("dlls").join("onnxruntime.dll");

        let profile = env::var("PROFILE").unwrap(); 
        let target_dir = Path::new(&manifest_dir).join("target").join(profile);

        let _ = fs::create_dir_all(&target_dir);

        let dest_path = target_dir.join("onnxruntime.dll");

        if dll_path.exists() {
             fs::copy(&dll_path, &dest_path).expect("Failed to copy onnxruntime.dll");
             println!("cargo:warning=Copied onnxruntime.dll to {:?}", dest_path);
        } else {
             println!("cargo:warning=Warning: onnxruntime.dll not found at {:?}", dll_path);
        }
    }
}