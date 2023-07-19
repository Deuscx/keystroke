// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;

use app::commands;
use app::listen_key;
use app::menu;
use commands::get_handlers;
fn main() {

    let mut tauri_app = tauri::Builder::default();

    #[cfg(not(target_os = "macos"))]
    {
        use menu::{get_system_tray, system_tray_handle};

        let system_tray = get_system_tray();
        tauri_app = tauri_app
            .system_tray(system_tray)
            .on_system_tray_event(system_tray_handle);
    }


    tauri_app
        // .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            let handle = app.handle();
            listen_key::create_device_query_listener(handle);
            Ok(())
        })
        .invoke_handler(get_handlers())
        // This is required to get tray-relative positions to work
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                if(event.window().title().unwrap() == "Settings") {
                    return;
                }
                event.window().hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
