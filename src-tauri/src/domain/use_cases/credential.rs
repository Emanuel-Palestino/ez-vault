use crate::domain::entities::Credential;
use crate::domain::traits::credential_repository::CredentialRepository;
use crate::VaultError;

pub struct CredentialUseCases<R: CredentialRepository> {
    repository: R,
}

impl<R: CredentialRepository> CredentialUseCases<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_credentials_by_app_id(
        &self,
        app_id: String,
    ) -> Result<Vec<Credential>, VaultError> {
        self.repository.get_credentials_by_app_id(app_id).await
    }

    pub async fn create_credential(
        &self,
        app_id: String,
        context: String,
        username: String,
        password: Option<String>,
        url: Option<String>,
        note: String,
    ) -> Result<(), VaultError> {
        let now_ts = chrono::Utc::now().timestamp_millis();
        let uuid = uuid::Uuid::new_v4().to_string();

        let credential = Credential {
            id: uuid,
            app_id,
            context,
            username,
            password,
            url,
            note,
            created_at_ts: now_ts,
            updated_at_ts: now_ts,
            deleted: false,
        };

        self.repository.store_credential(credential).await
    }

    pub async fn delete_credential(&self, id: String) -> Result<(), VaultError> {
        self.repository.soft_delete_credential(id).await
    }
}
