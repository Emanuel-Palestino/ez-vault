use crate::domain::use_cases::environment::EnvironmentUseCases;
use crate::errors::VaultError;
use crate::implementations::repositories::turso_env_repo::TursoEnvironmentRepo;
use crate::interfaces::IStorage;
use crate::services::TursoStorage;
use crate::types::EnvironmentCreate;
use tauri::async_runtime::Mutex;
use tauri::Manager;

use turso::Builder;

pub struct VaultApp {
    version: &'static str,
    pub storage: TursoStorage,
    env_domain: EnvironmentUseCases<TursoEnvironmentRepo>,
}

impl VaultApp {
    pub async fn new(db_path: String) -> Result<VaultApp, VaultError> {
        let db = Builder::new_local(&db_path)
            .build()
            .await
            .map_err(|e| VaultError::Database(e.to_string()))?;

        let conn = db
            .connect()
            .map_err(|e| VaultError::Database(e.to_string()))?;

        let storage = TursoStorage::new(conn.clone());

        storage.init().await?;

        let env_repo = TursoEnvironmentRepo::new(conn);
        env_repo.init().await?;

        let env_domain = EnvironmentUseCases::new(env_repo);

        Ok(VaultApp {
            version: "0.1.0",
            storage,
            env_domain,
        })
    }

    pub fn version(&self) -> &str {
        self.version
    }

    pub async fn create_environment(&mut self, data: EnvironmentCreate) -> Result<(), VaultError> {
        self.env_domain
            .create_environment(data.name, data.note)
            .await?;
        Ok(())
    }

    pub async fn get_environments(
        &self,
    ) -> Result<Vec<crate::domain::entities::Environment>, VaultError> {
        self.env_domain.get_environments().await
    }

    pub async fn delete_environment(&mut self, id: String) -> Result<(), VaultError> {
        self.env_domain.delete_environment(id).await?;
        Ok(())
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
