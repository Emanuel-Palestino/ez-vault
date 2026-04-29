use turso::Builder;

use crate::{errors::VaultError, services::TursoStorage};

pub struct StorageBuilder {
    database_url: Option<String>,
}

impl StorageBuilder {
    pub fn new() -> StorageBuilder {
        StorageBuilder { database_url: None }
    }

    pub fn with_database_url(self, database_url: String) -> StorageBuilder {
        StorageBuilder {
            database_url: Some(database_url),
        }
    }

    pub async fn build_turso_storage(self) -> Result<TursoStorage, VaultError> {
        let url = self
            .database_url
            .ok_or_else(|| VaultError::Internal("Database URL not set".into()))?;

        let db = Builder::new_local(&url)
            .build()
            .await
            .map_err(|e| VaultError::Database(e.to_string()))?;

        let conn = db
            .connect()
            .map_err(|e| VaultError::Database(e.to_string()))?;

        Ok(TursoStorage::new(conn))
    }
}
