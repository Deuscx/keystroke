// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn get_handlers() -> Box<dyn Fn(tauri::Invoke<tauri::Wry>) + Send + Sync> {
    Box::new(tauri::generate_handler![greet])
}
