mod storage_builder;
pub mod web_commands;

mod app;
use app::VaultApp;

use tauri::Manager;
pub fn main_setup(
) -> impl Fn(&mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    |app| {
        app.manage(VaultApp::new());
        Ok(())
    }
}
