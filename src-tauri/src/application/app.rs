use super::storage_builder::StorageBuilder;
use crate::interfaces::IStorage;
use crate::services::TursoStorage;

pub struct VaultApp {
    version: &'static str,
    // TODO: Make storage private and add getter
    pub storage: TursoStorage,
}

impl VaultApp {
    pub async fn new(
        storage: Option<TursoStorage>,
    ) -> Result<VaultApp, Box<dyn std::error::Error>> {
        if storage.is_some() {
            return Ok(VaultApp {
                version: "0.1.0",
                storage: storage.unwrap(),
            });
        } else {
            let storage = StorageBuilder::new()
                .with_default_environment()
                .build_local_turso_storage()
                .await?;

            storage.init().await?;

            Ok(VaultApp {
                version: "0.1.0",
                storage: storage,
            })
        }
    }

    pub fn check(&self) {
        println!("Vault app running in version {}", self.version);
    }

    /* pub fn get_storage(&self) -> &InMemoryStorage {
        &self.storage
    } */
}

use tauri::Wry;
use serde_json::json;
use tauri::async_runtime::Mutex;
use tauri::Manager;
use tauri_plugin_store::StoreExt;
pub fn main_tauri_setup(
) -> impl Fn(&mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    |app| {
        // TODO: verify this block code, is it correct?
        tauri::async_runtime::block_on(async {
            let store = app.store("store.json")?;
            let storage_type = store.get("storage-type").expect("storage-type not set");
            println!("{}", storage_type);

            let storage = match storage_type.as_str() {
                Some("LOCAL") => {
                   None
                }
                Some("REMOTE") => {
                    let database_url = store.get("database_url").expect("database_url not set");
                    let database_token = store.get("database_token").expect("database_token not set");
                    let storage = StorageBuilder::new()
                        .with_database_url(database_url.to_string())
                        .with_database_token(database_token.to_string())
                        .build_remote_turso_storage()
                        .await?;
                    storage.init().await?;

                    Some(storage)
                }
                Some("REPLICA") => {
                    let database_url = store.get("database_url").expect("database_url not set");
                    let database_token = store.get("database_token").expect("database_token not set");
                    let storage = StorageBuilder::new()
                        .with_database_url(database_url.to_string())
                        .with_database_token(database_token.to_string())
                        .build_replica_turso_storage()
                        .await?;
                    storage.init().await?;

                    Some(storage)
                }
                _ => {
                    panic!("Unknown storage type");
                }
            };
            let vault_app = VaultApp::new(storage).await?;
            app.manage(Mutex::new(vault_app));
            Ok(())
        })
    }
}
