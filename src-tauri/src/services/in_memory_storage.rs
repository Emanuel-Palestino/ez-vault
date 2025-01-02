use crate::interfaces::IStorage;
use crate::types::{Environment, NewEnvironment};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct InMemoryStorage {
    pub data: Vec<Environment>,
}

impl IStorage for InMemoryStorage {
    fn store_environment(&mut self, environment: NewEnvironment) {
        let now = Utc::now().timestamp();
        self.data.push(Environment {
            id: Uuid::new_v4().to_string(),
            name: environment.name,
            note: environment.note,
            created_at_ts: now,
            updated_at_ts: now,
        });
    }

    fn get_environments(&self) -> Vec<Environment> {
        self.data.clone()
    }
}
