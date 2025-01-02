use crate::types::{Environment, NewEnvironment};

pub trait IStorage {
    fn store_environment(&mut self, environment: NewEnvironment);
    fn get_environments(&self) -> Vec<Environment>;
}
