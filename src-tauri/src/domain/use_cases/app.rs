use crate::domain::entities::App;
use crate::domain::traits::app_repository::AppRepository;
use crate::VaultError;

pub struct AppUseCases<R: AppRepository> {
    repository: R,
}

impl<R: AppRepository> AppUseCases<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_apps(&self) -> Result<Vec<App>, VaultError> {
        self.repository.get_apps().await
    }

    pub async fn create_app(
        &self,
        name: String,
        url: String,
        environment_id: String,
        labels: Vec<String>,
        note: String,
    ) -> Result<(), VaultError> {
        let now_ts = chrono::Utc::now().timestamp_millis();
        let uuid = uuid::Uuid::new_v4().to_string();

        let app = App {
            id: uuid,
            name,
            url,
            environment_id,
            labels,
            note,
            created_at_ts: now_ts,
            updated_at_ts: now_ts,
            deleted: false,
        };

        self.repository.store_app(app).await
    }

    pub async fn delete_app(&self, id: String) -> Result<(), VaultError> {
        self.repository.soft_delete_app(id).await
    }
}
