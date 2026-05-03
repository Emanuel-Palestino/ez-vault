use crate::domain::entities::Secret;
use crate::domain::traits::secret_repository::SecretRepository;
use crate::VaultError;

pub struct SecretUseCases<R: SecretRepository> {
    repository: R,
}

impl<R: SecretRepository> SecretUseCases<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_secrets_by_app_id(&self, app_id: String) -> Result<Vec<Secret>, VaultError> {
        self.repository.get_secrets_by_app_id(app_id).await
    }

    pub async fn create_secret(
        &self,
        app_id: String,
        key: String,
        value: String,
        note: String,
    ) -> Result<(), VaultError> {
        let now_ts = chrono::Utc::now().timestamp_millis();
        let uuid = uuid::Uuid::new_v4().to_string();

        let secret = Secret {
            id: uuid,
            app_id,
            key,
            value,
            note,
            created_at_ts: now_ts,
            updated_at_ts: now_ts,
            deleted: false,
        };

        self.repository.store_secret(secret).await
    }

    pub async fn delete_secret(&self, id: String) -> Result<(), VaultError> {
        self.repository.soft_delete_secret(id).await
    }
}
