// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod application;
mod interfaces;
mod services;
mod types;

use application::{main_tauri_setup, web_commands};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(main_tauri_setup())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            web_commands::ez_vault_check,
            web_commands::ez_vault_create_environment,
            web_commands::ez_vault_get_environments,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
