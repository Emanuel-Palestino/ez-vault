use std::{error, time::Duration};

use libsql::Builder;

use crate::{
    interfaces::IStorage,
    services::{InMemoryStorage, TursoStorage},
    types::NewEnvironment,
};

pub struct StorageBuilder {
    default_environment: Option<NewEnvironment>,
    database_name: String,
    database_url: Option<String>,
    database_token: Option<String>,
}

impl StorageBuilder {
    pub fn new() -> StorageBuilder {
        StorageBuilder {
            default_environment: None,
            database_name: "local.db".to_string(),
            database_url: None,
            database_token: None,
        }
    }

    pub fn with_default_environment(self) -> StorageBuilder {
        StorageBuilder {
            default_environment: Some(NewEnvironment {
                name: "default".to_string(),
                note: "Default environment".to_string(),
            }),
            ..self
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

    pub fn with_database_url(self, database_url: String) -> StorageBuilder {
        StorageBuilder {
            database_url: Some(database_url),
            ..self
        }
    }

    pub fn with_database_token(self, database_token: String) -> StorageBuilder {
        StorageBuilder {
            database_token: Some(database_token),
            ..self
        }
    }

    pub async fn build_local_turso_storage(self) -> Result<TursoStorage, Box<dyn error::Error>> {
        let db = Builder::new_local(self.database_name).build().await?;
        let conn = db.connect()?;

        Ok(TursoStorage { conn })
    }

    pub async fn build_remote_turso_storage(self) -> Result<TursoStorage, Box<dyn error::Error>> {
        let url = self.database_url.ok_or("Database URL not set")?;
        let token = self.database_token.ok_or("Database token not set")?;

        let db = Builder::new_remote(url, token).build().await?;
        let conn = db.connect()?;

        Ok(TursoStorage { conn })
    }

    pub async fn build_replica_turso_storage(self) -> Result<TursoStorage, Box<dyn error::Error>> {
        let url = self.database_url.ok_or("Database URL not set")?;
        let token = self.database_token.ok_or("Database token not set")?;

        let db = Builder::new_remote_replica(self.database_name, url, token)
            .sync_interval(Duration::from_secs(60 * 5))
            .build()
            .await?;
        let conn = db.connect()?;

        Ok(TursoStorage { conn })
    }
}
