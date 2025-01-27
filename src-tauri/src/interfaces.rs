use crate::types::*;

pub trait IStorage {
    async fn init(&self) -> Result<(), Box<dyn std::error::Error>>;

    fn store_environment(&mut self, environment: NewEnvironment);
    fn get_environments(&self) -> Vec<Environment>;

    fn store_app(&mut self, app: NewApp);
    async fn get_apps(&self) -> Vec<App>;

    fn store_port(&mut self, port: NewPort);
    fn get_ports(&self) -> Vec<Port>;
    fn get_ports_by_app_id(&self, app_id: String) -> Vec<Port>;

    fn store_credential(&mut self, credential: NewCredential);
    fn get_credentials_by_app_id(&self, app_id: String) -> Vec<Credential>;

    fn store_secret(&mut self, secret: NewSecret);
    fn get_secrets_by_app_id(&self, app_id: String) -> Vec<Secret>;
}
