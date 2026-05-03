use crate::application::types::{
    AppCreate, CertificateCreate, CredentialCreate, EnvironmentCreate, SecretCreate,
};
use crate::domain::entities::{App, Certificate, Credential, Environment, Secret};
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
use tauri::async_runtime::Mutex;
use tauri::Manager;

use turso::Builder;

pub struct VaultApp {
    version: &'static str,
    env_domain: EnvironmentUseCases<TursoEnvironmentRepo>,
    app_domain: AppUseCases<TursoAppRepo>,
    credential_domain: CredentialUseCases<TursoCredentialRepo>,
    secret_domain: SecretUseCases<TursoSecretRepo>,
    certificate_domain: CertificateUseCases<TursoCertificateRepo>,
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

        Ok(VaultApp {
            version: "0.1.0",
            env_domain: EnvironmentUseCases::new(env_repo),
            app_domain: AppUseCases::new(app_repo),
            credential_domain: CredentialUseCases::new(credential_repo),
            secret_domain: SecretUseCases::new(secret_repo),
            certificate_domain: CertificateUseCases::new(certificate_repo),
        })
    }

    pub fn version(&self) -> &str {
        self.version
    }

    pub async fn create_environment(&self, data: EnvironmentCreate) -> Result<(), VaultError> {
        self.env_domain
            .create_environment(data.name, data.note)
            .await
    }

    pub async fn get_environments(&self) -> Result<Vec<Environment>, VaultError> {
        self.env_domain.get_environments().await
    }

    pub async fn delete_environment(&self, id: String) -> Result<(), VaultError> {
        self.env_domain.delete_environment(id).await
    }

    pub async fn create_app(&self, data: AppCreate) -> Result<(), VaultError> {
        self.app_domain
            .create_app(data.name, data.url, data.environment_id, data.labels, data.note)
            .await
    }

    pub async fn get_apps(&self) -> Result<Vec<App>, VaultError> {
        self.app_domain.get_apps().await
    }

    pub async fn delete_app(&self, id: String) -> Result<(), VaultError> {
        self.app_domain.delete_app(id).await
    }

    pub async fn create_credential(&self, data: CredentialCreate) -> Result<(), VaultError> {
        self.credential_domain
            .create_credential(
                data.app_id,
                data.context,
                data.username,
                data.password,
                data.url,
                data.note,
            )
            .await
    }

    pub async fn get_credentials_by_app_id(
        &self,
        app_id: String,
    ) -> Result<Vec<Credential>, VaultError> {
        self.credential_domain
            .get_credentials_by_app_id(app_id)
            .await
    }

    pub async fn delete_credential(&self, id: String) -> Result<(), VaultError> {
        self.credential_domain.delete_credential(id).await
    }

    pub async fn create_secret(&self, data: SecretCreate) -> Result<(), VaultError> {
        self.secret_domain
            .create_secret(data.app_id, data.key, data.value, data.note)
            .await
    }

    pub async fn get_secrets_by_app_id(&self, app_id: String) -> Result<Vec<Secret>, VaultError> {
        self.secret_domain.get_secrets_by_app_id(app_id).await
    }

    pub async fn delete_secret(&self, id: String) -> Result<(), VaultError> {
        self.secret_domain.delete_secret(id).await
    }

    pub async fn create_certificate(&self, data: CertificateCreate) -> Result<(), VaultError> {
        self.certificate_domain
            .create_certificate(
                data.name,
                data.file_name,
                data.file_extension,
                data.value,
                data.environment_id,
                data.labels,
                data.note,
            )
            .await
    }

    pub async fn get_certificates_by_environment_id(
        &self,
        environment_id: String,
    ) -> Result<Vec<Certificate>, VaultError> {
        self.certificate_domain
            .get_certificates_by_environment_id(environment_id)
            .await
    }

    pub async fn delete_certificate(&self, id: String) -> Result<(), VaultError> {
        self.certificate_domain.delete_certificate(id).await
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
