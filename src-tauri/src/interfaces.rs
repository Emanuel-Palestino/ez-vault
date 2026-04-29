use crate::types::*;

pub trait IStorage {
    async fn init(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn store_environment(&mut self, environment: EnvironmentCreate);
    async fn get_environments(&self) -> Vec<Environment>;

    async fn store_app(&mut self, app: AppCreate);
    async fn get_apps(&self) -> Vec<App>;

    async fn store_credential(&mut self, credential: CredentialCreate);
    async fn get_credentials_by_app_id(&self, app_id: String) -> Vec<Credential>;

    async fn store_secret(&mut self, secret: SecretCreate);
    async fn get_secrets_by_app_id(&self, app_id: String) -> Vec<Secret>;

    async fn store_certificate(&mut self, certificate: CertificateCreate);
    async fn get_certificates_by_environment_id(
        &self,
        environment_id: String,
    ) -> Vec<Certificate>;
}
