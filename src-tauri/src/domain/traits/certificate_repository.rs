use crate::domain::entities::Certificate;
use crate::errors::VaultError;

pub trait CertificateRepository {
    async fn store_certificate(&self, data: Certificate) -> Result<(), VaultError>;
    async fn get_certificates_by_environment_id(&self, environment_id: String) -> Result<Vec<Certificate>, VaultError>;
    async fn soft_delete_certificate(&self, id: String) -> Result<(), VaultError>;
}
