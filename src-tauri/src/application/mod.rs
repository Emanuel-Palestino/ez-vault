mod storage_builder;
pub mod web_commands;

mod app;
use app::VaultApp;

use std::sync::Mutex;

use tauri::Manager;
pub fn main_setup(
) -> impl Fn(&mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    |app| {
        app.manage(Mutex::new(VaultApp::new()));
        Ok(())
    }
}
