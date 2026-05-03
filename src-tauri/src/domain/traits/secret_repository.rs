use crate::domain::entities::Secret;
use crate::errors::VaultError;

pub trait SecretRepository {
    async fn store_secret(&self, data: Secret) -> Result<(), VaultError>;
    async fn get_secrets_by_app_id(&self, app_id: String) -> Result<Vec<Secret>, VaultError>;
    async fn soft_delete_secret(&self, id: String) -> Result<(), VaultError>;
}
