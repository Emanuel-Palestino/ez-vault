use crate::types::{Environment, NewEnvironment};
use super::interfaces::IStorage;

#[derive(Debug, Clone)]
pub struct InMemoryStorage {
    pub data: Vec<Environment>,
}

impl IStorage for InMemoryStorage {
    fn store_environment(&mut self, environment: NewEnvironment) {
        self.data.push(Environment {
            id: "123".to_string(),
            name: environment.name,
            note: environment.note,
            created_at_ts: 123,
            updated_at_ts: 123,
        });
    }

    fn get_environments(&self) -> Vec<Environment> {
        self.data.clone()
    }
}
