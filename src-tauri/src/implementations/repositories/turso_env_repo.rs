use crate::domain::entities::Environment;
use crate::domain::traits::env_repository::EnvironmentRepository;
use crate::errors::VaultError;

pub struct TursoEnvironmentRepo {
    conn: turso::Connection,
}

impl TursoEnvironmentRepo {
    pub fn new(conn: turso::Connection) -> Self {
        Self { conn }
    }
}

impl TursoEnvironmentRepo {
    pub async fn init(&self) -> Result<(), VaultError> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS environments (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    note TEXT NOT NULL DEFAULT '',
                    created_at_ts INTEGER NOT NULL,
                    updated_at_ts INTEGER NOT NULL,
                    deleted INTEGER NOT NULL DEFAULT 0
                )",
                (),
            )
            .await
            .map_err(VaultError::from)?;

        Ok(())
    }
}

impl EnvironmentRepository for TursoEnvironmentRepo {
    async fn store_environment(&self, data: Environment) -> Result<(), VaultError> {
        self.conn
            .execute(
                "INSERT INTO environments (id, name, note, created_at_ts, updated_at_ts, deleted) VALUES (?, ?, ?, ?, ?, 0)",
                (data.id, data.name, data.note, data.created_at_ts, data.updated_at_ts),
            )
            .await
            .map_err(VaultError::from)?;

        Ok(())
    }

    async fn get_environments(&self) -> Result<Vec<Environment>, VaultError> {
        // cols: 0=id 1=name 2=note 3=created_at_ts 4=updated_at_ts 5=deleted
        let mut rows = self
            .conn
            .query(
                "SELECT id, name, note, created_at_ts, updated_at_ts, deleted FROM environments WHERE deleted = 0",
                (),
            )
            .await
            .map_err(VaultError::from)?;

        let mut environments = Vec::new();
        while let Some(row) = rows.next().await.map_err(VaultError::from)? {
            let id: String = row.get(0).map_err(VaultError::from)?;
            let name: String = row.get(1).map_err(VaultError::from)?;
            let note: String = row.get(2).map_err(VaultError::from)?;
            let created_at_ts: i64 = row.get(3).map_err(VaultError::from)?;
            let updated_at_ts: i64 = row.get(4).map_err(VaultError::from)?;

            environments.push(Environment {
                id,
                created_at_ts,
                updated_at_ts,
                name,
                note,
                deleted: false,
            });
        }

        Ok(environments)
    }

    async fn soft_delete_environment(&self, id: String) -> Result<(), VaultError> {
        self.conn
            .execute("UPDATE environments SET deleted = 1 WHERE id = ?", (id,))
            .await
            .map_err(VaultError::from)?;

        Ok(())
    }
}
