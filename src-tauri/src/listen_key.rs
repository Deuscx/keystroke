
use rdev::{listen, Event,EventType,Key};
use tauri::Manager;

mod utils;
pub fn create_device_query_listener(handle: tauri::AppHandle) {

    std::thread::spawn(move || {
        let callback = move |event: Event| {
            match event.event_type {
                EventType::KeyRelease(key) => {
                    handle.emit_to("main", "KeyRelease",  utils::get_key(key) );
                }
                EventType::KeyPress(key) => {
                    println!("{:?}", key);
                    // handle.emit_to("main", "keypress","");
                }
                _ => println!("event ignored")
            }

        };

        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    });
}