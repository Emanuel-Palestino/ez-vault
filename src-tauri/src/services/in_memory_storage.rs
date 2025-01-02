use crate::types::Environment;
use super::interfaces::IStorage;

#[derive(Debug, Clone)]
pub struct InMemoryStorage {
    pub data: Vec<Environment>,
}

impl IStorage for InMemoryStorage {
    fn store_environment(&mut self, environment: Environment) {
        println!("Storing environment: {:?}", environment);
        self.data.push(environment);
    }

    fn get_environments(&self) -> Vec<Environment> {
        self.data.clone()
    }
}
