use crate::domain::entities::Credential;
use crate::domain::traits::credential_repository::CredentialRepository;
use crate::errors::VaultError;

pub struct TursoCredentialRepo {
    conn: turso::Connection,
}

impl TursoCredentialRepo {
    pub fn new(conn: turso::Connection) -> Self {
        Self { conn }
    }

    pub async fn init(&self) -> Result<(), VaultError> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS credentials (
                    id TEXT PRIMARY KEY,
                    app_id TEXT NOT NULL,
                    context TEXT NOT NULL,
                    username TEXT NOT NULL,
                    password TEXT,
                    url TEXT,
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

impl CredentialRepository for TursoCredentialRepo {
    async fn store_credential(&self, data: Credential) -> Result<(), VaultError> {
        self.conn
            .execute(
                "INSERT INTO credentials (id, app_id, context, username, password, url, note, created_at_ts, updated_at_ts, deleted) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0)",
                (data.id, data.app_id, data.context, data.username, data.password, data.url, data.note, data.created_at_ts, data.updated_at_ts),
            )
            .await
            .map_err(VaultError::from)?;

        Ok(())
    }

    async fn get_credentials_by_app_id(&self, app_id: String) -> Result<Vec<Credential>, VaultError> {
        // cols: 0=id 1=app_id 2=context 3=username 4=password 5=url 6=note 7=created_at_ts 8=updated_at_ts 9=deleted
        let mut rows = self
            .conn
            .query(
                "SELECT id, app_id, context, username, password, url, note, created_at_ts, updated_at_ts, deleted FROM credentials WHERE app_id = ? AND deleted = 0",
                &[app_id],
            )
            .await
            .map_err(VaultError::from)?;

        let mut credentials = Vec::new();
        while let Some(row) = rows.next().await.map_err(VaultError::from)? {
            let id: String = row.get(0).map_err(VaultError::from)?;
            let app_id: String = row.get(1).map_err(VaultError::from)?;
            let context: String = row.get(2).map_err(VaultError::from)?;
            let username: String = row.get(3).map_err(VaultError::from)?;
            let password: Option<String> = row.get(4).map_err(VaultError::from)?;
            let url: Option<String> = row.get(5).map_err(VaultError::from)?;
            let note: String = row.get(6).map_err(VaultError::from)?;
            let created_at_ts: i64 = row.get(7).map_err(VaultError::from)?;
            let updated_at_ts: i64 = row.get(8).map_err(VaultError::from)?;

            credentials.push(Credential {
                id,
                created_at_ts,
                updated_at_ts,
                app_id,
                context,
                username,
                password,
                url,
                note,
                deleted: false,
            });
        }

        Ok(credentials)
    }

    async fn soft_delete_credential(&self, id: String) -> Result<(), VaultError> {
        let now = chrono::Utc::now().timestamp_millis();
        self.conn
            .execute(
                "UPDATE credentials SET deleted = 1, updated_at_ts = ? WHERE id = ?",
                (now, id),
            )
            .await
            .map_err(VaultError::from)?;
        Ok(())
    }
}
