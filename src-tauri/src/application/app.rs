use super::storage_builder::StorageBuilder;
use crate::interfaces::IStorage;
use crate::services::TursoStorage;
use crate::types::StorageType;

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

    pub async fn update_storage_type(
        &mut self,
        storage_type: StorageType,
        url: Option<String>,
        token: Option<String>,
    ) {
        self.storage = match storage_type {
            StorageType::LOCAL => StorageBuilder::new()
                .with_default_environment()
                .build_local_turso_storage()
                .await
                .unwrap(),
            StorageType::REMOTE => StorageBuilder::new()
                .with_database_url(url.unwrap())
                .with_database_token(token.unwrap())
                .build_remote_turso_storage()
                .await
                .unwrap(),
            StorageType::REPLICA => StorageBuilder::new()
                .with_database_url(url.unwrap())
                .with_database_token(token.unwrap())
                .build_replica_turso_storage()
                .await
                .unwrap(),
        };

        self.storage.init().await.unwrap();
    }

    /* pub fn get_storage(&self) -> &InMemoryStorage {
        &self.storage
    } */
}

use serde_json::Value;
use tauri::async_runtime::Mutex;
use tauri::Manager;
use tauri_plugin_store::StoreExt;
pub fn main_tauri_setup(
) -> impl Fn(&mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    |app| {
        let store = app.store("store.json")?;
        let storage_type_value = store
            .get("storageType")
            .unwrap_or(Value::String("LOCAL".to_string()));
        let storage_type_str = storage_type_value
            .as_str()
            .expect("storage type is not a string");
        let storage_type = StorageType::from_str(storage_type_str);

        // TODO: verify this block code, is it correct?
        tauri::async_runtime::block_on(async {
            let storage = match storage_type {
                StorageType::LOCAL => None,
                StorageType::REMOTE => {
                    let url_value = store.get("databaseUrl").expect("databaseUrl not found");
                    let url_str = url_value.as_str().expect("databaseUrl is not a string");
                    let token_value = store.get("databaseToken").expect("databaseToken not found");
                    let token_str = token_value.as_str().expect("databaseToken is not a string");

                    let storage = StorageBuilder::new()
                        .with_database_url(url_str.to_string())
                        .with_database_token(token_str.to_string())
                        .build_remote_turso_storage()
                        .await?;
                    Some(storage)
                }
                StorageType::REPLICA => {
                    let url_value = store.get("databaseUrl").expect("databaseUrl not found");
                    let url_str = url_value.as_str().expect("databaseUrl is not a string");
                    let token_value = store.get("databaseToken").expect("databaseToken not found");
                    let token_str = token_value.as_str().expect("databaseToken is not a string");

                    let storage = StorageBuilder::new()
                        .with_database_url(url_str.to_string())
                        .with_database_token(token_str.to_string())
                        .build_replica_turso_storage()
                        .await?;
                    Some(storage)
                }
            };

            let vault_app = VaultApp::new(storage).await?;
            app.manage(Mutex::new(vault_app));
            Ok(())
        })
    }
}
