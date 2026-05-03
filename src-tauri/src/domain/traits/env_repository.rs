use crate::domain::entities::Environment;
use crate::errors::VaultError;

pub trait EnvironmentRepository {
    async fn store_environment(&self, data: Environment) -> Result<(), VaultError>;
    async fn get_environments(&self) -> Result<Vec<Environment>, VaultError>;
    async fn soft_delete_environment(&self, id: String) -> Result<(), VaultError>;
}
