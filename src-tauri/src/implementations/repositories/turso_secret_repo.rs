use crate::domain::entities::Secret;
use crate::domain::traits::secret_repository::SecretRepository;
use crate::errors::VaultError;

pub struct TursoSecretRepo {
    conn: turso::Connection,
}

impl TursoSecretRepo {
    pub fn new(conn: turso::Connection) -> Self {
        Self { conn }
    }

    pub async fn init(&self) -> Result<(), VaultError> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS secrets (
                    id TEXT PRIMARY KEY,
                    app_id TEXT NOT NULL,
                    key TEXT NOT NULL,
                    value TEXT NOT NULL,
                    note TEXT NOT NULL DEFAULT '',
                    created_at_ts INTEGER NOT NULL,
                    updated_at_ts INTEGER NOT NULL,
                    deleted INTEGER NOT NULL DEFAULT 0,
                    FOREIGN KEY (app_id) REFERENCES apps (id) ON DELETE CASCADE
                )",
                (),
            )
            .await
            .map_err(VaultError::from)?;

        Ok(())
    }
}

impl SecretRepository for TursoSecretRepo {
    async fn store_secret(&self, data: Secret) -> Result<(), VaultError> {
        self.conn
            .execute(
                "INSERT INTO secrets (id, app_id, key, value, note, created_at_ts, updated_at_ts, deleted) VALUES (?, ?, ?, ?, ?, ?, ?, 0)",
                (data.id, data.app_id, data.key, data.value, data.note, data.created_at_ts, data.updated_at_ts),
            )
            .await
            .map_err(VaultError::from)?;

        Ok(())
    }

    async fn get_secrets_by_app_id(&self, app_id: String) -> Result<Vec<Secret>, VaultError> {
        // cols: 0=id 1=app_id 2=key 3=value 4=note 5=created_at_ts 6=updated_at_ts 7=deleted
        let mut rows = self
            .conn
            .query(
                "SELECT id, app_id, key, value, note, created_at_ts, updated_at_ts, deleted FROM secrets WHERE app_id = ? AND deleted = 0",
                &[app_id],
            )
            .await
            .map_err(VaultError::from)?;

        let mut secrets = Vec::new();
        while let Some(row) = rows.next().await.map_err(VaultError::from)? {
            let id: String = row.get(0).map_err(VaultError::from)?;
            let app_id: String = row.get(1).map_err(VaultError::from)?;
            let key: String = row.get(2).map_err(VaultError::from)?;
            let value: String = row.get(3).map_err(VaultError::from)?;
            let note: String = row.get(4).map_err(VaultError::from)?;
            let created_at_ts: i64 = row.get(5).map_err(VaultError::from)?;
            let updated_at_ts: i64 = row.get(6).map_err(VaultError::from)?;

            secrets.push(Secret {
                id,
                created_at_ts,
                updated_at_ts,
                app_id,
                key,
                value,
                note,
                deleted: false,
            });
        }

        Ok(secrets)
    }

    async fn soft_delete_secret(&self, id: String) -> Result<(), VaultError> {
        let now = chrono::Utc::now().timestamp_millis();
        self.conn
            .execute(
                "UPDATE secrets SET deleted = 1, updated_at_ts = ? WHERE id = ?",
                (now, id),
            )
            .await
            .map_err(VaultError::from)?;
        Ok(())
    }
}
