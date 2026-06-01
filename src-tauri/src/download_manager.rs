use tokio::io::AsyncWriteExt;
use serde::Deserialize;
use core::arch;
use std::path::PathBuf;
use zip::ZipArchive;
use tauri::Manager;


#[derive(Deserialize)]
enum ModelSelection {
    Standard(StandardModel),
    Piper(PiperModel),
    Voiceover(VoiceoverModel),
}


#[derive(Deserialize)]
struct FullRegistry {
    main_models: Vec<StandardModel>,
    embedder: StandardModel,
    whisper: Vec<StandardModel>,
    piper: PiperModel,
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
struct PiperModel {
    value: String,
    download_url_windows: String,
    download_url_linux: String,
    download_url_macos: String,
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
            ModelSelection::Piper(registry.piper.clone())
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

            download_file(&model.download_url, &save_path).await.map_err(|e| e)?;
        },
        ModelSelection::Piper(piper) => {
            let save_path = app_data_dir.join(piper.folder_path);

            match std::env::consts::OS {
                "windows" => {
                    let file_name = PathBuf::from(format!("{}.zip", piper.file_name));

                    download_file(&piper.download_url_windows, &save_path.join(file_name.clone())).await.map_err(|e| e)?;
                    extract_archive(&save_path.join(file_name.clone()), &save_path).await.map_err(|e| e)?;

                    tokio::fs::remove_file(save_path.join(file_name)).await.map_err(|e| e.to_string())?;
                },
                "linux" => {
                    let file_name = PathBuf::from(format!("{}.tar.gz", piper.file_name));

                    download_file(&piper.download_url_linux, &save_path.join(file_name.clone())).await.map_err(|e| e)?;
                    extract_archive(&save_path.join(file_name.clone()), &save_path).await.map_err(|e| e)?;

                    tokio::fs::remove_file(save_path.join(file_name)).await.map_err(|e| e.to_string())?;
                },
                "macos" => {
                    let file_name = PathBuf::from(format!("{}.tar.gz", piper.file_name));

                    download_file(&piper.download_url_macos, &save_path.join(file_name.clone())).await.map_err(|e| e)?;
                    extract_archive(&save_path.join(file_name.clone()), &save_path).await.map_err(|e| e)?;

                    tokio::fs::remove_file(save_path.join(file_name)).await.map_err(|e| e.to_string())?;
                },
                _ => {
                    return Err("Unsupported OS".to_string());
                }
            }
        },
        ModelSelection::Voiceover(voiceover) => {
            let save_path = app_data_dir.join(voiceover.folder_path);
            let save_path_onnx = save_path.join(voiceover.file_name_onnx);
            let save_path_json = save_path.join(voiceover.file_name_json);

            download_file(&voiceover.download_url_onnx, &save_path_onnx).await.map_err(|e| e)?;
            download_file(&voiceover.download_url_json, &save_path_json).await.map_err(|e| e)?;
        },
    }

    Ok(())
}


async fn download_file(url: &str, save_path: &PathBuf) -> Result<(), String> {
    let save_folder = save_path.parent().expect("Couldn't reach the parent directory of save path");

    tokio::fs::create_dir_all(&save_folder).await.map_err(|e| e.to_string())?;
    tokio::fs::remove_file(save_path).await.ok();

    let tmp_path = PathBuf::from(format!("{}.tmp", save_path.display()));

    let mut response = reqwest::get(url).await.map_err(|e| e.to_string())?;
    let mut file = tokio::fs::File::create(&tmp_path).await.map_err(|e| e.to_string())?;

    while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
        file.write_all(&chunk).await.map_err(|e| e.to_string())?;
    }

    tokio::fs::rename(&tmp_path, save_path).await.map_err(|e| e.to_string())?;

    Ok(())
}


async fn extract_archive(archieve_path: &PathBuf, target_dir: &PathBuf) -> Result<(), String> {
    match std::env::consts::OS {
        "windows" => {
            let archieve_file = std::fs::File::open(archieve_path).map_err(|e| e.to_string())?;
            let mut archieve = ZipArchive::new(archieve_file).map_err(|e| e.to_string())?;
            archieve.extract(target_dir).map_err(|e| e.to_string())?;
        },
        _ => {
            let tar_gz = std::fs::File::open(archieve_path).map_err(|e| e.to_string())?;
            let tar = flate2::read::GzDecoder::new(tar_gz);
            let mut archieve = tar::Archive::new(tar);
            archieve.unpack(target_dir).map_err(|e| e.to_string())?;
        },
    }

    Ok(())
}