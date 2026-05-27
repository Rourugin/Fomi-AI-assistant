use serde::Deserialize;


#[derive(Deserialize)]
struct FullRegistry {
    main_models: Vec<StandardModel>,
    embedder: StandardModel,
    whisper: Vec<StandardModel>,
    piper: StandardModel,
    voiceover: Vec<VoiceoverModel>,
}

#[derive(Deserialize)]
struct StandardModel {
    value: String,
    download_url: String,
    file_name: String,
    folder_path: String,
}

#[derive(Deserialize)]
struct VoiceoverModel {
    value: String,
    download_url_onnx: String,
    download_url_json: String,
    file_name_onnx: String,
    file_name_json: String,
    folder_path: String
}


#[tauri::command]
pub async fn start_download(component_type: String, model_id: String) -> Result<(), String> {
    const url: &str = "https://raw.githubusercontent.com/Rourugin/Fomi-AI-assistant/refs/heads/main/src-tauri/models_registry.json";
    let json_text = reqwest::get(url)
        .await
        .expect("Failed to get json by URL")
        .text()
        .await
        .expect("Failed to convert json to text");

    let registry: FullRegistry = serde_json::from_str(&json_text)
        .expect("Failed to get registry from json text");

    Ok(())
}