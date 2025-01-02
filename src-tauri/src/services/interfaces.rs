use crate::types::Environment;

pub trait IStorage {
    fn store_environment(&mut self, environment: Environment);
    fn get_environments(&self) -> Vec<Environment>;
}
