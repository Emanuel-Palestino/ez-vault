use std::path::PathBuf;

use crate::domain::entities::Configuration;
use crate::domain::traits::configuration_service::ConfigurationService;
use crate::errors::VaultError;

pub struct JsonFileConfigService {
    path: PathBuf,
}

impl JsonFileConfigService {
    pub fn new(app_data_dir: PathBuf) -> Self {
        Self { path: app_data_dir.join("config.json") }
    }
}

impl ConfigurationService for JsonFileConfigService {
    async fn is_configured(&self) -> bool {
        if !self.path.exists() {
            return false;
        }
        self.get_configuration().await.is_ok()
    }

    async fn get_configuration(&self) -> Result<Configuration, VaultError> {
        let content = std::fs::read_to_string(&self.path)
            .map_err(|e| VaultError::NotConfigured(e.to_string()))?;

        serde_json::from_str::<Configuration>(&content)
            .map_err(|e| VaultError::NotConfigured(e.to_string()))
    }

    async fn save_configuration(&self, data: Configuration) -> Result<(), VaultError> {
        let content = serde_json::to_string_pretty(&data)
            .map_err(|e| VaultError::Internal(e.to_string()))?;

        std::fs::write(&self.path, content)
            .map_err(|e| VaultError::Internal(e.to_string()))
    }
}
