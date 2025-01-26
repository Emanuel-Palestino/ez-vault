use super::storage_builder::StorageBuilder;
use crate::services::TursoStorage;

pub struct VaultApp {
    version: &'static str,
    // TODO: Make storage private and add getter
    pub storage: TursoStorage,
}

impl VaultApp {
    pub async fn new() -> Result<VaultApp, Box<dyn std::error::Error>> {
        let storage = StorageBuilder::new()
            .with_default_environment()
            .with_database_url("test.db".to_string())
            .build_turso_storage()
            .await?;

        Ok(VaultApp {
            version: "0.1.0",
            storage: storage,
        })
    }

    pub fn check(&self) {
        println!("Vault app running in version {}", self.version);
    }

    /* pub fn get_storage(&self) -> &InMemoryStorage {
        &self.storage
    } */
}

use tauri::async_runtime::Mutex;
use tauri::Manager;
pub fn main_tauri_setup(
) -> impl Fn(&mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    |app| {
        // TODO: verify this block code, is it correct?
        tauri::async_runtime::block_on(async {
            let vault_app = VaultApp::new().await?;
            app.manage(Mutex::new(vault_app));
            Ok(())
        })
    }
}
