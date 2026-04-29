use crate::interfaces::IStorage;
use crate::types::*;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct InMemoryStorage {
    pub environments: Vec<Environment>,
    pub apps: Vec<App>,
    pub credentials: Vec<Credential>,
    pub secrets: Vec<Secret>,
    pub certificates: Vec<Certificate>,
}

impl IStorage for InMemoryStorage {
    async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("InMemoryStorage init");
        Ok(())
    }

    async fn store_environment(&mut self, environment: EnvironmentCreate) {
        let now = Utc::now().timestamp();
        self.environments.push(Environment {
            id: Uuid::new_v4().to_string(),
            created_at_ts: now,
            updated_at_ts: now,
            name: environment.name,
            note: environment.note,
            deleted: false,
        });
    }

    async fn get_environments(&self) -> Vec<Environment> {
        self.environments.clone()
    }

    async fn store_app(&mut self, app: AppCreate) {
        let now = Utc::now().timestamp();
        self.apps.push(App {
            id: Uuid::new_v4().to_string(),
            created_at_ts: now,
            updated_at_ts: now,
            name: app.name,
            url: app.url,
            environment_id: app.environment_id,
            labels: app.labels,
            note: app.note,
            deleted: false,
        });
    }

    async fn get_apps(&self) -> Vec<App> {
        self.apps.clone()
    }

    async fn store_credential(&mut self, credential: CredentialCreate) {
        let now = Utc::now().timestamp();
        self.credentials.push(Credential {
            id: Uuid::new_v4().to_string(),
            created_at_ts: now,
            updated_at_ts: now,
            app_id: credential.app_id,
            context: credential.context,
            username: credential.username,
            password: credential.password,
            url: credential.url,
            note: credential.note,
            deleted: false,
        });
    }

    async fn get_credentials_by_app_id(&self, app_id: String) -> Vec<Credential> {
        self.credentials
            .iter()
            .filter(|credential| credential.app_id == app_id)
            .cloned()
            .collect()
    }

    async fn store_secret(&mut self, secret: SecretCreate) {
        let now = Utc::now().timestamp();
        self.secrets.push(Secret {
            id: Uuid::new_v4().to_string(),
            created_at_ts: now,
            updated_at_ts: now,
            app_id: secret.app_id,
            key: secret.key,
            value: secret.value,
            note: secret.note,
            deleted: false,
        });
    }

    async fn get_secrets_by_app_id(&self, app_id: String) -> Vec<Secret> {
        self.secrets
            .iter()
            .filter(|secret| secret.app_id == app_id)
            .cloned()
            .collect()
    }

    async fn store_certificate(&mut self, certificate: CertificateCreate) {
        let now = Utc::now().timestamp();
        self.certificates.push(Certificate {
            id: Uuid::new_v4().to_string(),
            created_at_ts: now,
            updated_at_ts: now,
            name: certificate.name,
            file_name: certificate.file_name,
            file_extension: certificate.file_extension,
            value: certificate.value,
            environment_id: certificate.environment_id,
            labels: certificate.labels,
            note: certificate.note,
            deleted: false,
        });
    }

    async fn get_certificates_by_environment_id(
        &self,
        environment_id: String,
    ) -> Vec<Certificate> {
        self.certificates
            .iter()
            .filter(|certificate| certificate.environment_id == environment_id)
            .cloned()
            .collect()
    }
}
