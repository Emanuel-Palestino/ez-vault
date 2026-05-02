use crate::errors::VaultError;
use crate::interfaces::IStorage;
use crate::types::*;

pub struct TursoStorage {
    conn: turso::Connection,
}

impl TursoStorage {
    pub fn new(conn: turso::Connection) -> Self {
        TursoStorage { conn }
    }

    // cols: 0=id 1=name 2=url 3=environment_id 4=note 5=created_at_ts 6=updated_at_ts 7=deleted
    async fn load_app(&self, app_id: &str) -> Result<App, VaultError> {
        let mut app_rows = self
            .conn
            .query(
                "SELECT id, name, url, environment_id, note, created_at_ts, updated_at_ts, deleted FROM apps WHERE id = ?",
                &[app_id],
            )
            .await
            .map_err(VaultError::from)?;

        let app_row = app_rows
            .next()
            .await
            .map_err(VaultError::from)?
            .ok_or_else(|| VaultError::NotFound(format!("app {app_id}")))?;

        let id: String = app_row.get(0).map_err(VaultError::from)?;
        let name: String = app_row.get(1).map_err(VaultError::from)?;
        let url: String = app_row.get(2).map_err(VaultError::from)?;
        let environment_id: String = app_row.get(3).map_err(VaultError::from)?;
        let note: String = app_row.get(4).map_err(VaultError::from)?;
        let created_at_ts: i64 = app_row.get(5).map_err(VaultError::from)?;
        let updated_at_ts: i64 = app_row.get(6).map_err(VaultError::from)?;
        let deleted: bool = app_row
            .get::<i64>(7)
            .map(|v| v != 0)
            .map_err(VaultError::from)?;

        let mut label_rows = self
            .conn
            .query(
                "SELECT label FROM app_labels WHERE app_id = ?",
                &[id.as_str()],
            )
            .await
            .map_err(VaultError::from)?;

        let mut labels = Vec::new();
        while let Some(label_row) = label_rows.next().await.map_err(VaultError::from)? {
            let label: String = label_row.get(0).map_err(VaultError::from)?;
            labels.push(label);
        }

        Ok(App {
            id,
            created_at_ts,
            updated_at_ts,
            name,
            url,
            environment_id,
            labels,
            note,
            deleted,
        })
    }
}

impl IStorage for TursoStorage {
    async fn init(&self) -> Result<(), VaultError> {
        self.conn
            .execute_batch(
                r#"
                CREATE TABLE IF NOT EXISTS environments (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    note TEXT NOT NULL DEFAULT '',
                    created_at_ts INTEGER NOT NULL,
                    updated_at_ts INTEGER NOT NULL,
                    deleted INTEGER NOT NULL DEFAULT 0
                );

                CREATE TABLE IF NOT EXISTS apps (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    url TEXT NOT NULL,
                    environment_id TEXT NOT NULL,
                    note TEXT NOT NULL DEFAULT '',
                    created_at_ts INTEGER NOT NULL,
                    updated_at_ts INTEGER NOT NULL,
                    deleted INTEGER NOT NULL DEFAULT 0
                );

                CREATE TABLE IF NOT EXISTS app_labels (
                    app_id TEXT NOT NULL,
                    label TEXT NOT NULL,
                    FOREIGN KEY (app_id) REFERENCES apps (id) ON DELETE CASCADE,
                    PRIMARY KEY (app_id, label)
                );

                CREATE TABLE IF NOT EXISTS credentials (
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
                );

                CREATE TABLE IF NOT EXISTS secrets (
                    id TEXT PRIMARY KEY,
                    app_id TEXT NOT NULL,
                    key TEXT NOT NULL,
                    value TEXT NOT NULL,
                    note TEXT NOT NULL DEFAULT '',
                    created_at_ts INTEGER NOT NULL,
                    updated_at_ts INTEGER NOT NULL,
                    deleted INTEGER NOT NULL DEFAULT 0,
                    FOREIGN KEY (app_id) REFERENCES apps (id) ON DELETE CASCADE
                );

                CREATE TABLE IF NOT EXISTS certificates (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    file_name TEXT NOT NULL,
                    file_extension TEXT NOT NULL,
                    value TEXT NOT NULL,
                    environment_id TEXT NOT NULL,
                    note TEXT NOT NULL DEFAULT '',
                    created_at_ts INTEGER NOT NULL,
                    updated_at_ts INTEGER NOT NULL,
                    deleted INTEGER NOT NULL DEFAULT 0,
                    FOREIGN KEY (environment_id) REFERENCES environments (id) ON DELETE CASCADE
                );

                CREATE TABLE IF NOT EXISTS certificate_labels (
                    certificate_id TEXT NOT NULL,
                    label TEXT NOT NULL,
                    FOREIGN KEY (certificate_id) REFERENCES certificates (id) ON DELETE CASCADE,
                    PRIMARY KEY (certificate_id, label)
                );

                CREATE TABLE IF NOT EXISTS vault_users (
                    id TEXT PRIMARY KEY,
                    email TEXT NOT NULL UNIQUE,
                    salt TEXT NOT NULL,
                    encrypted_vault_key TEXT NOT NULL,
                    created_at_ts INTEGER NOT NULL
                );

                CREATE TABLE IF NOT EXISTS vault_invites (
                    id TEXT PRIMARY KEY,
                    invitee_email TEXT NOT NULL,
                    code_hash TEXT NOT NULL,
                    salt TEXT NOT NULL,
                    encrypted_vault_key TEXT NOT NULL,
                    created_at_ts INTEGER NOT NULL,
                    expires_at_ts INTEGER NOT NULL,
                    used INTEGER NOT NULL DEFAULT 0
                );
            "#,
            )
            .await
            .map_err(VaultError::from)?;

        Ok(())
    }

    async fn store_app(&mut self, data: AppCreate) -> Result<(), VaultError> {
        let now = chrono::Utc::now().timestamp();
        let uuid = uuid::Uuid::new_v4().to_string();

        self.conn
            .execute(
                "INSERT INTO apps (id, name, url, environment_id, note, created_at_ts, updated_at_ts, deleted) VALUES (?, ?, ?, ?, ?, ?, ?, 0)",
                (uuid.clone(), data.name, data.url, data.environment_id, data.note, now, now),
            )
            .await
            .map_err(VaultError::from)?;

        for label in data.labels {
            self.conn
                .execute(
                    "INSERT INTO app_labels (app_id, label) VALUES (?, ?)",
                    (uuid.clone(), label),
                )
                .await
                .map_err(VaultError::from)?;
        }

        Ok(())
    }

    async fn get_apps(&self) -> Result<Vec<App>, VaultError> {
        // cols: 0=id
        let mut rows = self
            .conn
            .query("SELECT id FROM apps WHERE deleted = 0", ())
            .await
            .map_err(VaultError::from)?;

        let mut apps = Vec::new();
        while let Some(row) = rows.next().await.map_err(VaultError::from)? {
            let id: String = row.get(0).map_err(VaultError::from)?;
            apps.push(self.load_app(&id).await?);
        }

        Ok(apps)
    }

    async fn delete_app(&mut self, id: String) -> Result<(), VaultError> {
        let now = chrono::Utc::now().timestamp();
        self.conn
            .execute(
                "UPDATE apps SET deleted = 1, updated_at_ts = ? WHERE id = ?",
                (now, id),
            )
            .await
            .map_err(VaultError::from)?;
        Ok(())
    }

    async fn store_credential(&mut self, data: CredentialCreate) -> Result<(), VaultError> {
        let now = chrono::Utc::now().timestamp();
        let uuid = uuid::Uuid::new_v4().to_string();

        self.conn
            .execute(
                "INSERT INTO credentials (id, app_id, context, username, password, url, note, created_at_ts, updated_at_ts, deleted) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0)",
                (uuid, data.app_id, data.context, data.username, data.password, data.url, data.note, now, now),
            )
            .await
            .map_err(VaultError::from)?;

        Ok(())
    }

    async fn get_credentials_by_app_id(
        &self,
        app_id: String,
    ) -> Result<Vec<Credential>, VaultError> {
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

    async fn delete_credential(&mut self, id: String) -> Result<(), VaultError> {
        let now = chrono::Utc::now().timestamp();
        self.conn
            .execute(
                "UPDATE credentials SET deleted = 1, updated_at_ts = ? WHERE id = ?",
                (now, id),
            )
            .await
            .map_err(VaultError::from)?;
        Ok(())
    }

    async fn store_secret(&mut self, data: SecretCreate) -> Result<(), VaultError> {
        let now = chrono::Utc::now().timestamp();
        let uuid = uuid::Uuid::new_v4().to_string();

        self.conn
            .execute(
                "INSERT INTO secrets (id, app_id, key, value, note, created_at_ts, updated_at_ts, deleted) VALUES (?, ?, ?, ?, ?, ?, ?, 0)",
                (uuid, data.app_id, data.key, data.value, data.note, now, now),
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

    async fn delete_secret(&mut self, id: String) -> Result<(), VaultError> {
        let now = chrono::Utc::now().timestamp();
        self.conn
            .execute(
                "UPDATE secrets SET deleted = 1, updated_at_ts = ? WHERE id = ?",
                (now, id),
            )
            .await
            .map_err(VaultError::from)?;
        Ok(())
    }

    async fn store_certificate(&mut self, data: CertificateCreate) -> Result<(), VaultError> {
        let now = chrono::Utc::now().timestamp();
        let uuid = uuid::Uuid::new_v4().to_string();

        self.conn
            .execute(
                "INSERT INTO certificates (id, name, file_name, file_extension, value, environment_id, note, created_at_ts, updated_at_ts, deleted) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0)",
                (uuid.clone(), data.name, data.file_name, data.file_extension, data.value, data.environment_id, data.note, now, now),
            )
            .await
            .map_err(VaultError::from)?;

        for label in data.labels {
            self.conn
                .execute(
                    "INSERT INTO certificate_labels (certificate_id, label) VALUES (?, ?)",
                    (uuid.clone(), label),
                )
                .await
                .map_err(VaultError::from)?;
        }

        Ok(())
    }

    async fn get_certificates_by_environment_id(
        &self,
        environment_id: String,
    ) -> Result<Vec<Certificate>, VaultError> {
        // cols: 0=id 1=name 2=file_name 3=file_extension 4=value 5=environment_id 6=note 7=created_at_ts 8=updated_at_ts 9=deleted
        let mut rows = self
            .conn
            .query(
                "SELECT id, name, file_name, file_extension, value, environment_id, note, created_at_ts, updated_at_ts, deleted FROM certificates WHERE environment_id = ? AND deleted = 0",
                &[environment_id],
            )
            .await
            .map_err(VaultError::from)?;

        let mut certificates = Vec::new();
        while let Some(row) = rows.next().await.map_err(VaultError::from)? {
            let id: String = row.get(0).map_err(VaultError::from)?;
            let name: String = row.get(1).map_err(VaultError::from)?;
            let file_name: String = row.get(2).map_err(VaultError::from)?;
            let file_extension: String = row.get(3).map_err(VaultError::from)?;
            let value: String = row.get(4).map_err(VaultError::from)?;
            let environment_id: String = row.get(5).map_err(VaultError::from)?;
            let note: String = row.get(6).map_err(VaultError::from)?;
            let created_at_ts: i64 = row.get(7).map_err(VaultError::from)?;
            let updated_at_ts: i64 = row.get(8).map_err(VaultError::from)?;

            let mut label_rows = self
                .conn
                .query(
                    "SELECT label FROM certificate_labels WHERE certificate_id = ?",
                    &[id.as_str()],
                )
                .await
                .map_err(VaultError::from)?;

            let mut labels = Vec::new();
            while let Some(label_row) = label_rows.next().await.map_err(VaultError::from)? {
                let label: String = label_row.get(0).map_err(VaultError::from)?;
                labels.push(label);
            }

            certificates.push(Certificate {
                id,
                name,
                file_name,
                file_extension,
                value,
                environment_id,
                labels,
                note,
                created_at_ts,
                updated_at_ts,
                deleted: false,
            });
        }

        Ok(certificates)
    }

    async fn delete_certificate(&mut self, id: String) -> Result<(), VaultError> {
        let now = chrono::Utc::now().timestamp();
        self.conn
            .execute(
                "UPDATE certificates SET deleted = 1, updated_at_ts = ? WHERE id = ?",
                (now, id),
            )
            .await
            .map_err(VaultError::from)?;
        Ok(())
    }
}

// Auth-specific methods — not part of IStorage trait
impl TursoStorage {
    #[allow(dead_code)]
    pub async fn vault_has_users(&self) -> Result<bool, VaultError> {
        let mut rows = self
            .conn
            .query("SELECT COUNT(*) FROM vault_users", ())
            .await
            .map_err(VaultError::from)?;
        let row = rows.next().await.map_err(VaultError::from)?;
        let count: i64 = row
            .ok_or_else(|| VaultError::Internal("count query returned no rows".into()))?
            .get(0)
            .map_err(VaultError::from)?;
        Ok(count > 0)
    }

    pub async fn create_vault_user(
        &mut self,
        email: &str,
        salt: &str,
        encrypted_vault_key: &str,
    ) -> Result<(), VaultError> {
        let now = chrono::Utc::now().timestamp();
        let id = uuid::Uuid::new_v4().to_string();
        self.conn
            .execute(
                "INSERT INTO vault_users (id, email, salt, encrypted_vault_key, created_at_ts) VALUES (?, ?, ?, ?, ?)",
                (id, email, salt, encrypted_vault_key, now),
            )
            .await
            .map_err(VaultError::from)?;
        Ok(())
    }

    // cols: 0=email 1=salt 2=encrypted_vault_key
    pub async fn get_vault_user(&self, email: &str) -> Result<VaultUserRow, VaultError> {
        let mut rows = self
            .conn
            .query(
                "SELECT email, salt, encrypted_vault_key FROM vault_users WHERE email = ?",
                &[email],
            )
            .await
            .map_err(VaultError::from)?;

        let row = rows
            .next()
            .await
            .map_err(VaultError::from)?
            .ok_or_else(|| VaultError::NotFound(format!("vault user {email}")))?;

        Ok(VaultUserRow {
            email: row.get(0).map_err(VaultError::from)?,
            salt: row.get(1).map_err(VaultError::from)?,
            encrypted_vault_key: row.get(2).map_err(VaultError::from)?,
        })
    }

    pub async fn create_invite(
        &mut self,
        invitee_email: &str,
        code_hash: &str,
        salt: &str,
        encrypted_vault_key: &str,
        expires_at_ts: i64,
    ) -> Result<(), VaultError> {
        let now = chrono::Utc::now().timestamp();
        let id = uuid::Uuid::new_v4().to_string();
        self.conn
            .execute(
                "INSERT INTO vault_invites (id, invitee_email, code_hash, salt, encrypted_vault_key, created_at_ts, expires_at_ts, used) VALUES (?, ?, ?, ?, ?, ?, ?, 0)",
                (id, invitee_email, code_hash, salt, encrypted_vault_key, now, expires_at_ts),
            )
            .await
            .map_err(VaultError::from)?;
        Ok(())
    }

    // cols: 0=id 1=salt 2=encrypted_vault_key
    // Marks the invite as used atomically.
    pub async fn consume_invite(
        &mut self,
        invitee_email: &str,
        code_hash: &str,
    ) -> Result<InviteRow, VaultError> {
        let now = chrono::Utc::now().timestamp();

        let mut rows = self
            .conn
            .query(
                "SELECT id, salt, encrypted_vault_key FROM vault_invites WHERE invitee_email = ? AND code_hash = ? AND used = 0 AND expires_at_ts > ?",
                (invitee_email, code_hash, now),
            )
            .await
            .map_err(VaultError::from)?;

        let row = rows
            .next()
            .await
            .map_err(VaultError::from)?
            .ok_or_else(|| VaultError::Auth("invalid or expired invite code".into()))?;

        let invite_id: String = row.get(0).map_err(VaultError::from)?;
        let salt: String = row.get(1).map_err(VaultError::from)?;
        let encrypted_vault_key: String = row.get(2).map_err(VaultError::from)?;

        self.conn
            .execute(
                "UPDATE vault_invites SET used = 1 WHERE id = ?",
                &[invite_id.as_str()],
            )
            .await
            .map_err(VaultError::from)?;

        Ok(InviteRow {
            salt,
            encrypted_vault_key,
        })
    }
}
