use super::storage_builder::StorageBuilder;
use crate::services::InMemoryStorage;

pub struct VaultApp {
    version: &'static str,
    // TODO: Make storage private and add getter
    pub storage: InMemoryStorage,
}

impl VaultApp {
    pub fn new() -> VaultApp {
        let storage = StorageBuilder::new().build_in_memory_storage();

        VaultApp {
            version: "0.1.0",
            storage,
        }
    }

    pub fn check(&self) {
        println!("Vault app running in version {}", self.version);
    }

    /* pub fn get_storage(&self) -> &InMemoryStorage {
        &self.storage
    } */
}
