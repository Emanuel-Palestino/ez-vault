use std::error;

use turso::Builder;

use crate::{interfaces::IStorage, services::{InMemoryStorage, TursoStorage}, types::EnvironmentCreate};

pub struct StorageBuilder {
    default_environment: Option<EnvironmentCreate>,
    database_url: Option<String>,
}

impl StorageBuilder {
    pub fn new() -> StorageBuilder {
        StorageBuilder {
            default_environment: None,
            database_url: None,
        }
    }

    pub fn with_default_environment(self) -> StorageBuilder {
        StorageBuilder {
            default_environment: Some(EnvironmentCreate {
                name: "default".to_string(),
                note: "Default environment".to_string(),
            }),
            ..self
        }
    }

    pub async fn build_in_memory_storage(self) -> InMemoryStorage {
        let mut storage = InMemoryStorage {
            environments: Vec::new(),
            apps: Vec::new(),
            credentials: Vec::new(),
            secrets: Vec::new(),
            certificates: Vec::new(),
        };

        if let Some(default_environment) = self.default_environment {
            storage.store_environment(default_environment).await;
        }

        storage
    }

    pub fn with_database_url(self, database_url: String) -> StorageBuilder {
        StorageBuilder {
            database_url: Some(database_url),
            ..self
        }
    }

    pub async fn build_turso_storage(self) -> Result<TursoStorage, Box<dyn error::Error>> {
        let url = self.database_url.ok_or("Database URL not set")?;

        let db = Builder::new_local(&url).build().await?;
        let conn = db.connect()?;

        Ok(TursoStorage {
            conn,
        })
    }
}
