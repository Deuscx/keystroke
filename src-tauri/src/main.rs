#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use rdev::{listen, Event};


fn main() {
    println!("start listening on");

            // This will block.
            if let Err(error) = listen(callback) {
                println!("Error: {:?}", error)
            };
        
            fn callback(event: Event) {
                println!("My callback {:?}", event);
                match event.name {
                    Some(string) => println!("User wrote {:?}", string),
                    None => (),
                }
            }      
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    

}
