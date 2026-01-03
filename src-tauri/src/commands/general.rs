#[tauri::command]
pub async fn greet(name: String) -> String {
    format!("Hello, {}! You've been greeted from Fomi! <3", name)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn say_bye(input: String) -> String {
    if input.to_lowercase().contains("hello") {
        return format!("Bye!");
    } else {
        return format!("");
    };
}