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
            web_commands::command_check,
            web_commands::command_create_environment,
            web_commands::command_get_environments,
            web_commands::command_create_app,
            web_commands::command_get_apps,
            /* web_commands::command_create_port,
            web_commands::command_get_ports,
            web_commands::command_get_ports_by_app_id,
            web_commands::command_create_credential,
            web_commands::command_get_credentials_by_app_id,
            web_commands::command_create_secret,
            web_commands::command_get_secrets_by_app_id, */
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
