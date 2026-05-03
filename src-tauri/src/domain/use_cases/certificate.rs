use crate::domain::entities::Certificate;
use crate::domain::traits::certificate_repository::CertificateRepository;
use crate::VaultError;

pub struct CertificateUseCases<R: CertificateRepository> {
    repository: R,
}

impl<R: CertificateRepository> CertificateUseCases<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_certificates_by_environment_id(
        &self,
        environment_id: String,
    ) -> Result<Vec<Certificate>, VaultError> {
        self.repository
            .get_certificates_by_environment_id(environment_id)
            .await
    }

    pub async fn create_certificate(
        &self,
        name: String,
        file_name: String,
        file_extension: String,
        value: String,
        environment_id: String,
        labels: Vec<String>,
        note: String,
    ) -> Result<(), VaultError> {
        let now_ts = chrono::Utc::now().timestamp_millis();
        let uuid = uuid::Uuid::new_v4().to_string();

        let certificate = Certificate {
            id: uuid,
            name,
            file_name,
            file_extension,
            value,
            environment_id,
            labels,
            note,
            created_at_ts: now_ts,
            updated_at_ts: now_ts,
            deleted: false,
        };

        self.repository.store_certificate(certificate).await
    }

    pub async fn delete_certificate(&self, id: String) -> Result<(), VaultError> {
        self.repository.soft_delete_certificate(id).await
    }
}
