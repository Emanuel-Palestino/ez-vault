// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::Manager;

mod domain;
mod implementations;

mod application;
use application::VaultApp;
use application::commands;

fn tauri_setup(
) -> impl Fn(&mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    |app| {
        app.manage(VaultApp::new("0.1.0"));
        Ok(())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(tauri_setup())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::ez_vault_check
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
