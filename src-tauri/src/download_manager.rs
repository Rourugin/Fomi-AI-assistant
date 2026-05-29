use ort::editor::Model;
use serde::Deserialize;
use std::{path::{Path, PathBuf}, result};
use tauri::Manager;


#[derive(Deserialize)]
enum ModelSelection {
    Standard(StandardModel),
    Voiceover(VoiceoverModel),
}


#[derive(Deserialize)]
struct FullRegistry {
    main_models: Vec<StandardModel>,
    embedder: StandardModel,
    whisper: Vec<StandardModel>,
    piper: StandardModel,
    voiceover: Vec<VoiceoverModel>,
}

#[derive(Deserialize, Clone)]
struct StandardModel {
    value: String,
    download_url: String,
    file_name: String,
    folder_path: String,
}

#[derive(Deserialize, Clone)]
struct VoiceoverModel {
    value: String,
    download_url_onnx: String,
    download_url_json: String,
    file_name_onnx: String,
    file_name_json: String,
    folder_path: String
}


#[tauri::command]
pub async fn start_download(app: tauri::AppHandle, component_type: String, model_id: String) -> Result<(), String> {
    const REGISTRY_URL: &str = "https://raw.githubusercontent.com/Rourugin/Fomi-AI-assistant/refs/heads/main/src-tauri/models_registry.json";
    let json_text = reqwest::get(REGISTRY_URL)
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    let registry: FullRegistry = serde_json::from_str(&json_text)
        .map_err(|e| e.to_string())?;

    let selected_model = match component_type.as_str() {
        "llm" => {
            let found_model = registry.main_models.iter().find(|m| m.value == model_id);

            match found_model {
                Some(model) => (ModelSelection::Standard(model.clone())),
                None => return Err("Model not found in registry".to_string()),
            }
        },
        "embedder" => {
            ModelSelection::Standard(registry.embedder.clone())
        },
        "whisper" => {
            let found_whisper = registry.whisper.iter().find(|w| w.value == model_id);

            match found_whisper {
                Some(whisper) => (ModelSelection::Standard(whisper.clone())),
                None => return Err("Whisper not found in registry".to_string()),
            }
        },
        "piper" => {
            ModelSelection::Standard(registry.piper.clone())
        },
        "voiceover" => {
            let found_voiceover = registry.voiceover.iter().find(|v| v.value == model_id);

            match found_voiceover {
                Some(voiceover) => (ModelSelection::Voiceover(voiceover.clone())),
                None => return Err("Voiveover not found in registry".to_string()),
            }
        },
        _ => {
            return Err("Unknown component type".to_string());
        }
    };

    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    match selected_model {
        ModelSelection::Standard(model) => {
            let save_path = app_data_dir.join(model.folder_path).join(model.file_name);

            println!("Ready to download: {} here: {:?}", model.download_url, save_path);
        },
        ModelSelection::Voiceover(voiceover) => {
            let save_path = app_data_dir.join(voiceover.folder_path);
            let save_path_onnx = save_path.join(voiceover.file_name_onnx);
            let save_path_json = save_path.join(voiceover.file_name_json);

            println!("Ready to download: {} here: {:?}", voiceover.download_url_onnx, save_path_onnx);
            println!("Ready to download: {} here: {:?}", voiceover.download_url_json, save_path_json);
        },
    }

    Ok(())
}


async fn download_file(url: &str, save_path: &PathBuf) -> Result<(), String> {
    Ok(())
}