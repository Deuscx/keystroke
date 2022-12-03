#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod listen_key;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn main() {
    println!("start listening on");

    tauri::Builder::default().setup( |app|{
        let handle = app.handle();
        _ = handle.emit_all("test", "test");
        listen_key::create_device_query_listener(handle);
        Ok(())
    }).invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    

}
