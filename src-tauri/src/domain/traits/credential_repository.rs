use crate::domain::entities::Credential;
use crate::errors::VaultError;

pub trait CredentialRepository {
    async fn store_credential(&self, data: Credential) -> Result<(), VaultError>;
    async fn get_credentials_by_app_id(&self, app_id: String) -> Result<Vec<Credential>, VaultError>;
    async fn soft_delete_credential(&self, id: String) -> Result<(), VaultError>;
}
