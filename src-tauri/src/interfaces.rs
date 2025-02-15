use crate::types::*;

pub trait IStorage {
    async fn init(&self) -> Result<(), Box<dyn std::error::Error>>;

    async fn store_environment(&mut self, environment: NewEnvironment);
    async fn get_environments(&self) -> Vec<Environment>;

    async fn store_app(&mut self, app: NewApp);
    async fn get_apps(&self) -> Vec<App>;

    async fn store_port(&mut self, port: NewPort);
    async fn get_ports(&self) -> Vec<Port>;
    async fn get_ports_by_app_id(&self, app_id: String) -> Vec<Port>;

    async fn store_credential(&mut self, credential: NewCredential);
    async fn get_credentials_by_app_id(&self, app_id: String) -> Vec<Credential>;

    async fn store_secret(&mut self, secret: NewSecret);
    async fn get_secrets_by_app_id(&self, app_id: String) -> Vec<Secret>;
}
