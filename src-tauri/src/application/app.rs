use crate::domain::aggregate::VaultDomain;
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

type FinalVaultDomain = VaultDomain<
    TursoEnvironmentRepo,
    TursoAppRepo,
    TursoCredentialRepo,
    TursoSecretRepo,
    TursoCertificateRepo,
>;

pub struct VaultApp {
    version: &'static str,
    pub domain: FinalVaultDomain,
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

        let domain = VaultDomain::new(
            EnvironmentUseCases::new(env_repo),
            AppUseCases::new(app_repo),
            CredentialUseCases::new(credential_repo),
            SecretUseCases::new(secret_repo),
            CertificateUseCases::new(certificate_repo),
        );

        Ok(VaultApp {
            version: "0.1.0",
            domain,
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
