use crate::domain::interfaces::IStorage;
use crate::domain::types::Environment;

#[derive(Debug)]
pub struct InMemoryStorage {
    pub data: Vec<Environment>,
}

impl IStorage for InMemoryStorage {
    fn store_environment(&mut self, environment: Environment) {
        println!("Storing environment: {:?}", environment);
        self.data.push(environment);
    }
}
