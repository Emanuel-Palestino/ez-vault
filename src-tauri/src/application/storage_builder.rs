use crate::{interfaces::IStorage, services::InMemoryStorage, types::NewEnvironment};

pub struct StorageBuilder {
    default_environment: Option<NewEnvironment>,
}

impl StorageBuilder {
    pub fn new() -> StorageBuilder {
        StorageBuilder {
            default_environment: None,
        }
    }

    pub fn with_default_environment(self) -> StorageBuilder {
        StorageBuilder {
            default_environment: Some(NewEnvironment {
                name: "default".to_string(),
                note: "Default environment".to_string(),
            }),
        }
    }

    pub fn build_in_memory_storage(self) -> InMemoryStorage {
        let mut storage = InMemoryStorage {
            environments: Vec::new(),
            apps: Vec::new(),
            ports: Vec::new(),
            credentials: Vec::new(),
            secrets: Vec::new(),
        };

        if let Some(default_environment) = self.default_environment {
            storage.store_environment(default_environment);
        }

        storage
    }
}
