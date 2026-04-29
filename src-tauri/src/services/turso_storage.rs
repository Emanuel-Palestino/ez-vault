use crate::interfaces::IStorage;
use crate::types::*;

pub struct TursoStorage {
    pub conn: turso::Connection,
}

impl TursoStorage {
    async fn load_app(&self, app_id: String) -> App {
        let mut app_rows = self
            .conn
            .query("SELECT * FROM apps WHERE id = ?", &[app_id.clone()])
            .await
            .unwrap();

        let app_row = app_rows.next().await.unwrap().unwrap();
        let id: String = app_row.get(0).unwrap();
        let name: String = app_row.get(1).unwrap();
        let url: String = app_row.get(2).unwrap();
        let environment_id: String = app_row.get(3).unwrap();
        let note: String = app_row.get(4).unwrap();
        let created_at_ts: i64 = app_row.get(5).unwrap();
        let updated_at_ts: i64 = app_row.get(6).unwrap();

        let mut label_rows = self
            .conn
            .query("SELECT label FROM app_labels WHERE app_id = ?", &[id.clone()])
            .await
            .unwrap();
        let mut labels = Vec::new();
        while let Some(label_row) = label_rows.next().await.unwrap() {
            let label: String = label_row.get(0).unwrap();
            labels.push(label);
        }

        App {
            id,
            created_at_ts,
            updated_at_ts,
            name,
            url,
            environment_id,
            labels,
            note,
            deleted: false,
        }
    }
}

impl IStorage for TursoStorage {
    async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("TursoStorage init");

        self.conn
            .execute_batch(
                r#"
                -- Tabla de Entornos (Environments)
                CREATE TABLE IF NOT EXISTS environments (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    note TEXT,
                    created_at_ts INTEGER NOT NULL,
                    updated_at_ts INTEGER NOT NULL
                );

                -- Tabla de Aplicaciones (Apps)
                CREATE TABLE IF NOT EXISTS apps (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    url TEXT NOT NULL,
                    environment_id TEXT NOT NULL,
                    note TEXT,
                    created_at_ts INTEGER NOT NULL,
                    updated_at_ts INTEGER NOT NULL
                );

                -- Tabla de etiquetas (labels) para las aplicaciones (App tiene un array de labels)
                CREATE TABLE IF NOT EXISTS app_labels (
                    app_id TEXT NOT NULL,
                    label TEXT NOT NULL,
                    FOREIGN KEY (app_id) REFERENCES apps (id) ON DELETE CASCADE,
                    PRIMARY KEY (app_id, label)
                );

                -- Tabla de Credenciales (Credentials)
                CREATE TABLE IF NOT EXISTS credentials (
                    id TEXT PRIMARY KEY,
                    app_id TEXT NOT NULL,
                    context TEXT NOT NULL,
                    username TEXT NOT NULL,
                    password TEXT,
                    url TEXT,
                    note TEXT,
                    created_at_ts INTEGER NOT NULL,
                    updated_at_ts INTEGER NOT NULL,
                    FOREIGN KEY (app_id) REFERENCES apps (id) ON DELETE CASCADE
                );

                -- Tabla de Secretos (Secrets)
                CREATE TABLE IF NOT EXISTS secrets (
                    id TEXT PRIMARY KEY,
                    app_id TEXT NOT NULL,
                    key TEXT NOT NULL,
                    value TEXT NOT NULL,
                    note TEXT,
                    created_at_ts INTEGER NOT NULL,
                    updated_at_ts INTEGER NOT NULL,
                    FOREIGN KEY (app_id) REFERENCES apps (id) ON DELETE CASCADE
                );

                -- Tabla de Certificados (Certificates)
                CREATE TABLE IF NOT EXISTS certificates (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    file_name TEXT NOT NULL,
                    file_extension TEXT NOT NULL,
                    value TEXT NOT NULL,
                    environment_id TEXT NOT NULL,
                    note TEXT,
                    created_at_ts INTEGER NOT NULL,
                    updated_at_ts INTEGER NOT NULL,
                    FOREIGN KEY (environment_id) REFERENCES environments (id) ON DELETE CASCADE
                );

                -- Tabla de etiquetas (labels) para certificados
                CREATE TABLE IF NOT EXISTS certificate_labels (
                    certificate_id TEXT NOT NULL,
                    label TEXT NOT NULL,
                    FOREIGN KEY (certificate_id) REFERENCES certificates (id) ON DELETE CASCADE,
                    PRIMARY KEY (certificate_id, label)
                );
            "#,
            )
            .await?;

        Ok(())
    }

    async fn store_environment(&mut self, environment: EnvironmentCreate) {
        println!("TursoStorage store_environment");
        let now = chrono::Utc::now().timestamp();
        let uuid = uuid::Uuid::new_v4().to_string();

        let mut stmt = self.conn
        .prepare("INSERT INTO environments (id, name, note, created_at_ts, updated_at_ts) VALUES (?, ?, ?, ?, ?)")
        .await
        .unwrap();

        stmt.execute([
            uuid,
            environment.name,
            environment.note,
            now.to_string(),
            now.to_string(),
        ])
        .await
        .unwrap();
    }

    async fn get_environments(&self) -> Vec<Environment> {
        println!("TursoStorage get_environments");
        let mut rows = self
            .conn
            .query("SELECT * FROM environments", ())
            .await
            .unwrap();
        let mut environments = Vec::new();

        while let Some(row) = rows.next().await.unwrap() {
            let id: String = row.get(0).unwrap();
            let name: String = row.get(1).unwrap();
            let note: String = row.get(2).unwrap();
            let created_at_ts: i64 = row.get(3).unwrap();
            let updated_at_ts: i64 = row.get(4).unwrap();
            environments.push(Environment {
                id,
                created_at_ts,
                updated_at_ts,
                name,
                note,
                deleted: false,
            });
        }

        environments
    }

    async fn store_app(&mut self, app: AppCreate) {
        println!("TursoStorage store_app");

        let now = chrono::Utc::now().timestamp();
        let uuid = uuid::Uuid::new_v4().to_string();

        let mut stmt = self.conn
        .prepare("INSERT INTO apps (id, name, url, environment_id, note, created_at_ts, updated_at_ts) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .await
        .unwrap();

        stmt.execute([
            uuid.clone(),
            app.name,
            app.url,
            app.environment_id,
            app.note,
            now.to_string(),
            now.to_string(),
        ])
        .await
        .unwrap();

        for label in app.labels {
            let mut stmt = self
                .conn
                .prepare("INSERT INTO app_labels (app_id, label) VALUES (?, ?)")
                .await
                .unwrap();

            stmt.execute([uuid.clone(), label]).await.unwrap();
        }
    }

    async fn get_apps(&self) -> Vec<App> {
        println!("TursoStorage get_apps");
        let mut rows = self.conn.query("SELECT * FROM apps", ()).await.unwrap();
        let mut apps = Vec::new();

        while let Some(row) = rows.next().await.unwrap() {
            let id: String = row.get(0).unwrap();
            apps.push(self.load_app(id).await);
        }

        apps
    }

    async fn store_credential(&mut self, credential: CredentialCreate) {
        println!("TursoStorage store_credential");

        let now = chrono::Utc::now().timestamp();
        let uuid = uuid::Uuid::new_v4().to_string();

        let mut stmt = self.conn
        .prepare("INSERT INTO credentials (id, app_id, context, username, password, url, note, created_at_ts, updated_at_ts) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .await
        .unwrap();

        stmt.execute((
            uuid,
            credential.app_id,
            credential.context,
            credential.username,
            credential.password,
            credential.url,
            credential.note,
            now.to_string(),
            now.to_string(),
        ))
        .await
        .unwrap();
    }

    async fn get_credentials_by_app_id(&self, app_id: String) -> Vec<Credential> {
        println!("TursoStorage get_credentials_by_app_id");

        let mut rows = self
            .conn
            .query("SELECT * FROM credentials WHERE app_id = ?", &[app_id])
            .await
            .unwrap();
        let mut credentials = Vec::new();

        while let Some(row) = rows.next().await.unwrap() {
            let id: String = row.get(0).unwrap();
            let app_id: String = row.get(1).unwrap();
            let context: String = row.get(2).unwrap();
            let username: String = row.get(3).unwrap();
            let password: Option<String> = row.get(4).unwrap();
            let url: Option<String> = row.get(5).unwrap();
            let note: String = row.get(6).unwrap();
            let created_at_ts: i64 = row.get(7).unwrap();
            let updated_at_ts: i64 = row.get(8).unwrap();

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

        credentials
    }

    async fn store_secret(&mut self, secret: SecretCreate) {
        println!("TursoStorage store_secret");

        let now = chrono::Utc::now().timestamp();
        let uuid = uuid::Uuid::new_v4().to_string();

        let mut stmt = self.conn
        .prepare("INSERT INTO secrets (id, app_id, key, value, note, created_at_ts, updated_at_ts) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .await
        .unwrap();

        stmt.execute([
            uuid,
            secret.app_id,
            secret.key,
            secret.value,
            secret.note,
            now.to_string(),
            now.to_string(),
        ])
        .await
        .unwrap();
    }

    async fn get_secrets_by_app_id(&self, app_id: String) -> Vec<Secret> {
        println!("TursoStorage get_secrets_by_app_id");

        let mut rows = self
            .conn
            .query("SELECT * FROM secrets WHERE app_id = ?", &[app_id])
            .await
            .unwrap();
        let mut secrets = Vec::new();

        while let Some(row) = rows.next().await.unwrap() {
            let id: String = row.get(0).unwrap();
            let app_id: String = row.get(1).unwrap();
            let key: String = row.get(2).unwrap();
            let value: String = row.get(3).unwrap();
            let note: String = row.get(4).unwrap();
            let created_at_ts: i64 = row.get(5).unwrap();
            let updated_at_ts: i64 = row.get(6).unwrap();

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

        secrets
    }

    async fn store_certificate(&mut self, certificate: CertificateCreate) {
        println!("TursoStorage store_certificate");

        let now = chrono::Utc::now().timestamp();
        let uuid = uuid::Uuid::new_v4().to_string();

        let mut stmt = self
            .conn
            .prepare("INSERT INTO certificates (id, name, file_name, file_extension, value, environment_id, note, created_at_ts, updated_at_ts) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .await
            .unwrap();

        stmt.execute([
            uuid.clone(),
            certificate.name,
            certificate.file_name,
            certificate.file_extension,
            certificate.value,
            certificate.environment_id,
            certificate.note,
            now.to_string(),
            now.to_string(),
        ])
        .await
        .unwrap();

        for label in certificate.labels {
            let mut stmt = self
                .conn
                .prepare("INSERT INTO certificate_labels (certificate_id, label) VALUES (?, ?)")
                .await
                .unwrap();

            stmt.execute([uuid.clone(), label]).await.unwrap();
        }
    }

    async fn get_certificates_by_environment_id(
        &self,
        environment_id: String,
    ) -> Vec<Certificate> {
        println!("TursoStorage get_certificates_by_environment_id");

        let mut rows = self
            .conn
            .query(
                "SELECT * FROM certificates WHERE environment_id = ?",
                &[environment_id],
            )
            .await
            .unwrap();
        let mut certificates = Vec::new();

        while let Some(row) = rows.next().await.unwrap() {
            let id: String = row.get(0).unwrap();
            let name: String = row.get(1).unwrap();
            let file_name: String = row.get(2).unwrap();
            let file_extension: String = row.get(3).unwrap();
            let value: String = row.get(4).unwrap();
            let environment_id: String = row.get(5).unwrap();
            let note: String = row.get(6).unwrap();
            let created_at_ts: i64 = row.get(7).unwrap();
            let updated_at_ts: i64 = row.get(8).unwrap();

            let mut label_rows = self
                .conn
                .query(
                    "SELECT label FROM certificate_labels WHERE certificate_id = ?",
                    &[id.clone()],
                )
                .await
                .unwrap();
            let mut labels = Vec::new();
            while let Some(label_row) = label_rows.next().await.unwrap() {
                let label: String = label_row.get(0).unwrap();
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

        certificates
    }
}
