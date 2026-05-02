use turso::sync::Builder;

use crate::{errors::VaultError, services::TursoStorage};

pub struct StorageBuilder {
    local_replica_path: Option<String>,
    remote_url: Option<String>,
    auth_token: Option<String>,
}

impl StorageBuilder {
    pub fn new() -> StorageBuilder {
        StorageBuilder {
            local_replica_path: None,
            remote_url: None,
            auth_token: None,
        }
    }

    pub fn with_local_path(self, path: String) -> StorageBuilder {
        StorageBuilder {
            local_replica_path: Some(path),
            ..self
        }
    }

    pub fn with_remote_url(self, url: String) -> StorageBuilder {
        StorageBuilder {
            remote_url: Some(url),
            ..self
        }
    }

    pub fn with_auth_token(self, token: String) -> StorageBuilder {
        StorageBuilder {
            auth_token: Some(token),
            ..self
        }
    }

    pub async fn build_turso_storage(self) -> Result<TursoStorage, VaultError> {
        let url = self
            .remote_url
            .ok_or_else(|| VaultError::Internal("remote URL not set".into()))?;
        let token = self
            .auth_token
            .ok_or_else(|| VaultError::Internal("auth token not set".into()))?;
        let local_path = self
            .local_replica_path
            .ok_or_else(|| VaultError::Internal("local replica path not set".into()))?;

        let db = Builder::new_remote(&local_path)
            .with_remote_url(url)
            .with_auth_token(token)
            .build()
            .await
            .map_err(|e| VaultError::Database(e.to_string()))?;

        let conn = db
            .connect()
            .await
            .map_err(|e| VaultError::Database(e.to_string()))?;

        Ok(TursoStorage::new(conn))
    }
}
