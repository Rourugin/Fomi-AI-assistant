use serde::Deserialize;


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



    Ok(())
}