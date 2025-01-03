use crate::services::InMemoryStorage;

pub struct StorageBuilder;

impl StorageBuilder {
    pub fn new() -> StorageBuilder {
        StorageBuilder
    }

    pub fn build_in_memory_storage(self) -> InMemoryStorage {
        InMemoryStorage {
            environments: Vec::new(),
            apps: Vec::new(),
            ports: Vec::new(),
            credentials: Vec::new(),
            secrets: Vec::new(),
        }
    }
}
