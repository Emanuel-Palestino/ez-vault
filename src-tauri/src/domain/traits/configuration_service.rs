use crate::{domain::entities::Configuration, VaultError};

pub trait ConfigurationService {
    async fn is_configured(&self) -> bool;
    async fn save_configuration(&self, data: Configuration) -> Result<(), VaultError>;
    async fn get_configuration(&self) -> Result<Configuration, VaultError>;
}
