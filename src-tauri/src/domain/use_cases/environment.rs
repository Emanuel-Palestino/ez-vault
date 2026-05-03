use crate::domain::{entities::Environment, traits::env_repository::EnvironmentRepository};
use crate::VaultError;

pub struct EnvironmentUseCases<R: EnvironmentRepository> {
    repository: R,
}

impl<R: EnvironmentRepository> EnvironmentUseCases<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_environments(&self) -> Result<Vec<Environment>, VaultError> {
        self.repository.get_environments().await
    }

    pub async fn create_environment(&self, name: String, note: String) -> Result<(), VaultError> {
        let now_ts = chrono::Utc::now().timestamp_millis();
        let uuid = uuid::Uuid::new_v4().to_string();

        let env = Environment {
            id: uuid,
            name,
            note,
            created_at_ts: now_ts,
            updated_at_ts: now_ts,
            deleted: false,
        };

        self.repository.store_environment(env).await
    }

    pub async fn delete_environment(&self, id: String) -> Result<(), VaultError> {
        self.repository.soft_delete_environment(id).await
    }
}
