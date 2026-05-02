mod application;
mod domain;
mod errors;
mod implementations;
mod interfaces;
mod security;
mod services;
mod types;

pub use errors::VaultError;

use application::{main_tauri_setup, web_commands};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(main_tauri_setup())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Setup / session
            web_commands::command_check_setup_status,
            web_commands::command_create_vault,
            web_commands::command_join_vault,
            web_commands::command_unlock,
            web_commands::command_lock,
            web_commands::command_get_current_user,
            web_commands::command_invite_user,
            // Meta
            web_commands::command_get_version,
            // Environments
            web_commands::command_create_environment,
            web_commands::command_get_environments,
            web_commands::command_delete_environment,
            // Apps
            web_commands::command_create_app,
            web_commands::command_get_apps,
            web_commands::command_delete_app,
            // Credentials
            web_commands::command_create_credential,
            web_commands::command_get_credentials_by_app_id,
            web_commands::command_delete_credential,
            // Secrets
            web_commands::command_create_secret,
            web_commands::command_get_secrets_by_app_id,
            web_commands::command_delete_secret,
            // Certificates
            web_commands::command_create_certificate,
            web_commands::command_get_certificates_by_environment_id,
            web_commands::command_delete_certificate,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
