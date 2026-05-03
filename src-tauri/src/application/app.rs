use std::path::{Path, PathBuf};

use crate::domain::aggregate::VaultDomain;
use crate::domain::entities::Configuration;
use crate::domain::traits::configuration_service::ConfigurationService;
use crate::domain::use_cases::app::AppUseCases;
use crate::domain::use_cases::certificate::CertificateUseCases;
use crate::domain::use_cases::credential::CredentialUseCases;
use crate::domain::use_cases::environment::EnvironmentUseCases;
use crate::domain::use_cases::secret::SecretUseCases;
use crate::errors::VaultError;
use crate::implementations::repositories::turso_app_repo::TursoAppRepo;
use crate::implementations::repositories::turso_certificate_repo::TursoCertificateRepo;
use crate::implementations::repositories::turso_credential_repo::TursoCredentialRepo;
use crate::implementations::repositories::turso_env_repo::TursoEnvironmentRepo;
use crate::implementations::repositories::turso_secret_repo::TursoSecretRepo;
use crate::implementations::services::json_file_config_service::JsonFileConfigService;
use tauri::async_runtime::Mutex;
use tauri::Manager;

type FinalVaultDomain = VaultDomain<
    TursoEnvironmentRepo,
    TursoAppRepo,
    TursoCredentialRepo,
    TursoSecretRepo,
    TursoCertificateRepo,
>;

pub struct VaultApp {
    version: &'static str,
    app_data_dir: PathBuf,
    pub config_service: JsonFileConfigService,
    domain: Option<FinalVaultDomain>,
}

impl VaultApp {
    pub async fn new(app_data_dir: PathBuf) -> Result<VaultApp, VaultError> {
        let config_service = JsonFileConfigService::new(app_data_dir.clone());

        let domain = if config_service.is_configured().await {
            let config = config_service.get_configuration().await?;
            Some(Self::build_domain(&app_data_dir, config).await?)
        } else {
            None
        };

        Ok(VaultApp {
            version: "0.1.0",
            app_data_dir,
            config_service,
            domain,
        })
    }

    pub fn version(&self) -> &str {
        self.version
    }

    pub fn domain(&self) -> Result<&FinalVaultDomain, VaultError> {
        self.domain
            .as_ref()
            .ok_or(VaultError::NotConfigured("app not configured".into()))
    }

    pub async fn init_domain(&mut self) -> Result<(), VaultError> {
        let config = self.config_service.get_configuration().await?;
        self.domain = Some(Self::build_domain(&self.app_data_dir, config).await?);
        Ok(())
    }

    async fn build_domain(
        app_data_dir: &Path,
        config: Configuration,
    ) -> Result<FinalVaultDomain, VaultError> {
        let local_path = app_data_dir.join("vault.db").to_string_lossy().to_string();

        let db = turso::sync::Builder::new_remote(&local_path)
            .with_remote_url(&config.database_url)
            .with_auth_token(&config.database_token)
            .build()
            .await
            .map_err(|e| VaultError::Database(e.to_string()))?;

        let conn = db
            .connect()
            .await
            .map_err(|e| VaultError::Database(e.to_string()))?;

        let env_repo = TursoEnvironmentRepo::new(conn.clone());
        env_repo.init().await?;

        let app_repo = TursoAppRepo::new(conn.clone());
        app_repo.init().await?;

        let credential_repo = TursoCredentialRepo::new(conn.clone());
        credential_repo.init().await?;

        let secret_repo = TursoSecretRepo::new(conn.clone());
        secret_repo.init().await?;

        let certificate_repo = TursoCertificateRepo::new(conn);
        certificate_repo.init().await?;

        Ok(VaultDomain::new(
            EnvironmentUseCases::new(env_repo),
            AppUseCases::new(app_repo),
            CredentialUseCases::new(credential_repo),
            SecretUseCases::new(secret_repo),
            CertificateUseCases::new(certificate_repo),
        ))
    }
}

pub fn main_tauri_setup(
) -> impl Fn(&mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    |app| {
        tauri::async_runtime::block_on(async {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;

            let vault_app = VaultApp::new(app_data_dir).await?;
            app.manage(Mutex::new(vault_app));
            Ok(())
        })
    }
}
