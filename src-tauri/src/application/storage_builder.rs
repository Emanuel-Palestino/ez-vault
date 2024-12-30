use crate::implementations::InMemoryStorage;

pub struct StorageBuilder;

impl StorageBuilder {
  pub fn build() -> InMemoryStorage {
    InMemoryStorage { data: Vec::new() }
  }
}