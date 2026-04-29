use super::storage_builder::StorageBuilder;
use crate::errors::VaultError;
use crate::interfaces::IStorage;
use crate::services::TursoStorage;
use tauri::async_runtime::Mutex;
use tauri::Manager;

pub struct VaultApp {
    version: &'static str,
    pub storage: TursoStorage,
}

impl VaultApp {
    pub async fn new(db_path: String) -> Result<VaultApp, VaultError> {
        let storage = StorageBuilder::new()
            .with_database_url(db_path)
            .build_turso_storage()
            .await?;

        storage.init().await?;

        Ok(VaultApp {
            version: "0.1.0",
            storage,
        })
    }

    pub fn version(&self) -> &str {
        self.version
    }
}

pub fn main_tauri_setup(
) -> impl Fn(&mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    |app| {
        tauri::async_runtime::block_on(async {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;
            let db_path = app_data_dir.join("vault.db").to_string_lossy().to_string();

            let vault_app = VaultApp::new(db_path).await?;
            app.manage(Mutex::new(vault_app));
            Ok(())
        })
    }
}
