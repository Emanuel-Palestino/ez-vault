// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod domain;
mod implementations;

mod application;
use application::StorageBuilder;
use tauri::Manager;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = StorageBuilder::build();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
