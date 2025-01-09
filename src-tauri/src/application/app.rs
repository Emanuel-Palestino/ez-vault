use super::storage_builder::StorageBuilder;
use crate::interfaces::IStorage;

pub struct VaultApp {
    version: &'static str,
    // TODO: Make storage private and add getter
    pub storage: Box<dyn IStorage + Send>,
}

impl VaultApp {
    pub fn new() -> VaultApp {
        let storage = StorageBuilder::new()
            .with_default_environment()
            .with_database_url("lala.db".to_string())
            .build_turso_storage();

        VaultApp {
            version: "0.1.0",
            storage: Box::new(storage),
        }
    }

    pub fn check(&self) {
        println!("Vault app running in version {}", self.version);
    }

    /* pub fn get_storage(&self) -> &InMemoryStorage {
        &self.storage
    } */
}

use std::sync::Mutex;
use tauri::Manager;
pub fn main_tauri_setup(
) -> impl Fn(&mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    |app| {
        app.manage(Mutex::new(VaultApp::new()));
        Ok(())
    }
}
