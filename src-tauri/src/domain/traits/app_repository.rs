use crate::domain::entities::App;
use crate::errors::VaultError;

pub trait AppRepository {
    async fn store_app(&self, data: App) -> Result<(), VaultError>;
    async fn get_apps(&self) -> Result<Vec<App>, VaultError>;
    async fn soft_delete_app(&self, id: String) -> Result<(), VaultError>;
}
