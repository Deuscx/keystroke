use crate::app::key;
use rdev::{listen, Event, EventType, Key};
use tauri::Manager;

pub fn create_device_query_listener(handle: tauri::AppHandle) {

    std::thread::spawn(move || {
        let callback = move |event: Event| {
            if let EventType::KeyRelease(key) = event.event_type {
                let name = event.name.clone().unwrap_or("".to_string());
                handle.emit_to("main", "KeyRelease", name);
            }
            if let EventType::KeyPress(key) = event.event_type {
                let name = event.name.clone().unwrap_or("".to_string());

                let special_name = key::get_key(key);
                println!(
                    "My callback {:?}, {:?}, {:?}",
                    event.event_type, name, special_name
                );
                handle.emit_to(
                    "main",
                    "KeyPress",
                    if (special_name.ne("")) {
                        special_name
                    } else {
                        name
                    },
                );
            }
        };

        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    });
}
