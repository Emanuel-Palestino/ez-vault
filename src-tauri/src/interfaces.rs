use crate::errors::VaultError;
use crate::types::*;

pub trait IStorage {
    async fn init(&self) -> Result<(), VaultError>;

    async fn store_app(&mut self, data: AppCreate) -> Result<(), VaultError>;
    async fn get_apps(&self) -> Result<Vec<App>, VaultError>;
    async fn delete_app(&mut self, id: String) -> Result<(), VaultError>;

    async fn store_credential(&mut self, data: CredentialCreate) -> Result<(), VaultError>;
    async fn get_credentials_by_app_id(
        &self,
        app_id: String,
    ) -> Result<Vec<Credential>, VaultError>;
    async fn delete_credential(&mut self, id: String) -> Result<(), VaultError>;

    async fn store_secret(&mut self, data: SecretCreate) -> Result<(), VaultError>;
    async fn get_secrets_by_app_id(&self, app_id: String) -> Result<Vec<Secret>, VaultError>;
    async fn delete_secret(&mut self, id: String) -> Result<(), VaultError>;

    async fn store_certificate(&mut self, data: CertificateCreate) -> Result<(), VaultError>;
    async fn get_certificates_by_environment_id(
        &self,
        environment_id: String,
    ) -> Result<Vec<Certificate>, VaultError>;
    async fn delete_certificate(&mut self, id: String) -> Result<(), VaultError>;
}
