use crate::domain::entities::{App, Certificate, Credential, Environment, Secret};
use crate::domain::traits::app_repository::AppRepository;
use crate::domain::traits::certificate_repository::CertificateRepository;
use crate::domain::traits::credential_repository::CredentialRepository;
use crate::domain::traits::env_repository::EnvironmentRepository;
use crate::domain::traits::secret_repository::SecretRepository;
use crate::domain::use_cases::app::AppUseCases;
use crate::domain::use_cases::certificate::CertificateUseCases;
use crate::domain::use_cases::credential::CredentialUseCases;
use crate::domain::use_cases::environment::EnvironmentUseCases;
use crate::domain::use_cases::secret::SecretUseCases;
use crate::errors::VaultError;

pub struct VaultDomain<ER, AR, CR, SR, CertR>
where
    ER: EnvironmentRepository,
    AR: AppRepository,
    CR: CredentialRepository,
    SR: SecretRepository,
    CertR: CertificateRepository,
{
    env: EnvironmentUseCases<ER>,
    app: AppUseCases<AR>,
    credential: CredentialUseCases<CR>,
    secret: SecretUseCases<SR>,
    certificate: CertificateUseCases<CertR>,
}

impl<ER, AR, CR, SR, CertR> VaultDomain<ER, AR, CR, SR, CertR>
where
    ER: EnvironmentRepository,
    AR: AppRepository,
    CR: CredentialRepository,
    SR: SecretRepository,
    CertR: CertificateRepository,
{
    pub fn new(
        env: EnvironmentUseCases<ER>,
        app: AppUseCases<AR>,
        credential: CredentialUseCases<CR>,
        secret: SecretUseCases<SR>,
        certificate: CertificateUseCases<CertR>,
    ) -> Self {
        Self { env, app, credential, secret, certificate }
    }

    // --- Environment ---

    pub async fn create_environment(&self, name: String, note: String) -> Result<(), VaultError> {
        self.env.create_environment(name, note).await
    }

    pub async fn get_environments(&self) -> Result<Vec<Environment>, VaultError> {
        self.env.get_environments().await
    }

    pub async fn delete_environment(&self, id: String) -> Result<(), VaultError> {
        self.env.delete_environment(id).await
    }

    // --- App ---

    pub async fn create_app(
        &self,
        name: String,
        url: String,
        environment_id: String,
        labels: Vec<String>,
        note: String,
    ) -> Result<(), VaultError> {
        self.app.create_app(name, url, environment_id, labels, note).await
    }

    pub async fn get_apps(&self) -> Result<Vec<App>, VaultError> {
        self.app.get_apps().await
    }

    pub async fn delete_app(&self, id: String) -> Result<(), VaultError> {
        self.app.delete_app(id).await
    }

    // --- Credential ---

    pub async fn create_credential(
        &self,
        app_id: String,
        context: String,
        username: String,
        password: Option<String>,
        url: Option<String>,
        note: String,
    ) -> Result<(), VaultError> {
        self.credential
            .create_credential(app_id, context, username, password, url, note)
            .await
    }

    pub async fn get_credentials_by_app_id(
        &self,
        app_id: String,
    ) -> Result<Vec<Credential>, VaultError> {
        self.credential.get_credentials_by_app_id(app_id).await
    }

    pub async fn delete_credential(&self, id: String) -> Result<(), VaultError> {
        self.credential.delete_credential(id).await
    }

    // --- Secret ---

    pub async fn create_secret(
        &self,
        app_id: String,
        key: String,
        value: String,
        note: String,
    ) -> Result<(), VaultError> {
        self.secret.create_secret(app_id, key, value, note).await
    }

    pub async fn get_secrets_by_app_id(&self, app_id: String) -> Result<Vec<Secret>, VaultError> {
        self.secret.get_secrets_by_app_id(app_id).await
    }

    pub async fn delete_secret(&self, id: String) -> Result<(), VaultError> {
        self.secret.delete_secret(id).await
    }

    // --- Certificate ---

    pub async fn create_certificate(
        &self,
        name: String,
        file_name: String,
        file_extension: String,
        value: String,
        environment_id: String,
        labels: Vec<String>,
        note: String,
    ) -> Result<(), VaultError> {
        self.certificate
            .create_certificate(name, file_name, file_extension, value, environment_id, labels, note)
            .await
    }

    pub async fn get_certificates_by_environment_id(
        &self,
        environment_id: String,
    ) -> Result<Vec<Certificate>, VaultError> {
        self.certificate
            .get_certificates_by_environment_id(environment_id)
            .await
    }

    pub async fn delete_certificate(&self, id: String) -> Result<(), VaultError> {
        self.certificate.delete_certificate(id).await
    }
}
