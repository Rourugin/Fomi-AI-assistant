// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn greet(name: String) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(rename_all = "snake_case")]
async fn say_bye(input: String) -> String {
    if input.to_lowercase().contains("hello") {
        return format!("Bye!");
    } else {
        return format!("");
    };
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![say_bye, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}