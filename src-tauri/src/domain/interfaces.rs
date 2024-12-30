use crate::domain::types::Environment;

pub trait IStorage {
  fn store_environment(&mut self, environment: Environment);
}