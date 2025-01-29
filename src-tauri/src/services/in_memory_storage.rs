use crate::interfaces::IStorage;
use crate::types::*;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct InMemoryStorage {
    pub environments: Vec<Environment>,
    pub apps: Vec<App>,
    pub ports: Vec<Port>,
    pub credentials: Vec<Credential>,
    pub secrets: Vec<Secret>,
}

impl IStorage for InMemoryStorage {
    async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("InMemoryStorage init");
        Ok(())
    }

    async fn store_environment(&mut self, environment: NewEnvironment) {
        let now = Utc::now().timestamp();
        self.environments.push(Environment {
            id: Uuid::new_v4().to_string(),
            name: environment.name,
            note: environment.note,
            created_at_ts: now,
            updated_at_ts: now,
        });
    }

    async fn get_environments(&self) -> Vec<Environment> {
        self.environments.clone()
    }

    fn store_app(&mut self, app: NewApp) {
        let now = Utc::now().timestamp();
        self.apps.push(App {
            id: Uuid::new_v4().to_string(),
            name: app.name,
            url: app.url,
            note: app.note,
            environments: app
                .environment_ids
                .iter()
                .map(|env_id| {
                    self.environments
                        .iter()
                        .find(|env| env.id == *env_id)
                        .unwrap()
                        .clone()
                })
                .collect(),
            labels: app.labels,
            bounded_context: app.bounded_context,
            created_at_ts: now,
            updated_at_ts: now,
        });
    }

    async fn get_apps(&self) -> Vec<App> {
        self.apps.clone()
    }

    fn store_port(&mut self, port: NewPort) {
        let now = Utc::now().timestamp();
        self.ports.push(Port {
            id: Uuid::new_v4().to_string(),
            app: self
                .apps
                .iter()
                .find(|app| app.id == port.app_id)
                .unwrap()
                .clone(),
            value: port.value,
            note: port.note,
            created_at_ts: now,
            updated_at_ts: now,
        });
    }

    fn get_ports(&self) -> Vec<Port> {
        self.ports.clone()
    }

    fn get_ports_by_app_id(&self, app_id: String) -> Vec<Port> {
        self.ports
            .iter()
            .filter(|port| port.app.id == app_id)
            .cloned()
            .collect()
    }

    fn store_credential(&mut self, credential: NewCredential) {
        let now = Utc::now().timestamp();
        self.credentials.push(Credential {
            id: Uuid::new_v4().to_string(),
            app: self
                .apps
                .iter()
                .find(|app| app.id == credential.app_id)
                .unwrap()
                .clone(),
            username: credential.username,
            password: credential.password,
            note: credential.note,
            created_at_ts: now,
            updated_at_ts: now,
        });
    }

    fn get_credentials_by_app_id(&self, app_id: String) -> Vec<Credential> {
        self.credentials
            .iter()
            .filter(|credential| credential.app.id == app_id)
            .cloned()
            .collect()
    }

    fn store_secret(&mut self, secret: NewSecret) {
        let now = Utc::now().timestamp();
        self.secrets.push(Secret {
            id: Uuid::new_v4().to_string(),
            app: self
                .apps
                .iter()
                .find(|app| app.id == secret.app_id)
                .unwrap()
                .clone(),
            key: secret.key,
            value: secret.value,
            note: secret.note,
            created_at_ts: now,
            updated_at_ts: now,
        });
    }

    fn get_secrets_by_app_id(&self, app_id: String) -> Vec<Secret> {
        self.secrets
            .iter()
            .filter(|secret| secret.app.id == app_id)
            .cloned()
            .collect()
    }
}
