use crate::interfaces::IStorage;
use crate::types::*;

pub struct TursoStorage {
    pub conn: libsql::Connection,
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
                    note TEXT,
                    bounded_context TEXT NOT NULL,
                    created_at_ts INTEGER NOT NULL,
                    updated_at_ts INTEGER NOT NULL
                );

                -- Tabla intermedia para la relación muchos a muchos entre apps y environments
                CREATE TABLE IF NOT EXISTS app_environments (
                    app_id TEXT NOT NULL,
                    environment_id TEXT NOT NULL,
                    FOREIGN KEY (app_id) REFERENCES apps (id) ON DELETE CASCADE,
                    FOREIGN KEY (environment_id) REFERENCES environments (id) ON DELETE CASCADE,
                    PRIMARY KEY (app_id, environment_id)
                );

                -- Tabla de etiquetas (labels) para las aplicaciones (App tiene un array de labels)
                CREATE TABLE IF NOT EXISTS app_labels (
                    app_id TEXT NOT NULL,
                    label TEXT NOT NULL,
                    FOREIGN KEY (app_id) REFERENCES apps (id) ON DELETE CASCADE,
                    PRIMARY KEY (app_id, label)
                );

                -- Tabla de Puertos (Ports)
                CREATE TABLE IF NOT EXISTS ports (
                    id TEXT PRIMARY KEY,
                    app_id TEXT NOT NULL,
                    value INTEGER NOT NULL,
                    note TEXT,
                    created_at_ts INTEGER NOT NULL,
                    updated_at_ts INTEGER NOT NULL,
                    FOREIGN KEY (app_id) REFERENCES apps (id) ON DELETE CASCADE
                );

                -- Tabla de Credenciales (Credentials)
                CREATE TABLE IF NOT EXISTS credentials (
                    id TEXT PRIMARY KEY,
                    app_id TEXT NOT NULL,
                    username TEXT NOT NULL,
                    password TEXT NOT NULL,
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
            "#,
            )
            .await?;

        Ok(())
    }

    async fn store_environment(&mut self, environment: NewEnvironment) {
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
                name,
                note,
                created_at_ts,
                updated_at_ts,
            });
        }

        environments
    }

    async fn store_app(&mut self, app: NewApp) {
        println!("TursoStorage store_app");

        let now = chrono::Utc::now().timestamp();
        let uuid = uuid::Uuid::new_v4().to_string();

        let mut stmt = self.conn
        .prepare("INSERT INTO apps (id, name, url, note, bounded_context, created_at_ts, updated_at_ts) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .await
        .unwrap();

        stmt.execute([
            uuid.clone(),
            app.name,
            app.url,
            app.note,
            app.bounded_context,
            now.to_string(),
            now.to_string(),
        ])
        .await
        .unwrap();

        for environment_id in app.environment_ids {
            let mut stmt = self
                .conn
                .prepare("INSERT INTO app_environments (app_id, environment_id) VALUES (?, ?)")
                .await
                .unwrap();

            stmt.execute([uuid.clone(), environment_id]).await.unwrap();
        }

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
            let name: String = row.get(1).unwrap();
            let url: String = row.get(2).unwrap();
            let note: String = row.get(3).unwrap();
            let bounded_context: String = row.get(4).unwrap();
            let created_at_ts: i64 = row.get(5).unwrap();
            let updated_at_ts: i64 = row.get(6).unwrap();

            let mut env_rows = self.conn
                .query(
                    "SELECT e.* FROM environments e JOIN app_environments ae ON e.id =ae.environment_id WHERE ae.app_id = ?",
                    &[id.clone()],
                )
                .await
                .unwrap();
            let mut environments = Vec::new();
            while let Some(env_row) = env_rows.next().await.unwrap() {
                let env_id: String = env_row.get(0).unwrap();
                let env_name: String = env_row.get(1).unwrap();
                let env_note: String = env_row.get(2).unwrap();
                let env_created_at_ts: i64 = env_row.get(3).unwrap();
                let env_updated_at_ts: i64 = env_row.get(4).unwrap();
                environments.push(Environment {
                    id: env_id,
                    name: env_name,
                    note: env_note,
                    created_at_ts: env_created_at_ts,
                    updated_at_ts: env_updated_at_ts,
                });
            }

            let mut label_rows = self.conn
                .query("SELECT label FROM app_labels WHERE app_id = ?", &[id.clone()])
                .await
                .unwrap();
            let mut labels = Vec::new();
            while let Some(label_row) = label_rows.next().await.unwrap() {
                let label: String = label_row.get(0).unwrap();
                labels.push(label);
            }

            apps.push(App {
                id,
                name,
                url,
                note,
                bounded_context,
                created_at_ts,
                updated_at_ts,
                environments,
                labels,
            });
        }

        apps
    }

    fn store_port(&mut self, port: NewPort) {
        todo!()
    }

    fn get_ports(&self) -> Vec<Port> {
        todo!()
    }

    fn get_ports_by_app_id(&self, app_id: String) -> Vec<Port> {
        todo!()
    }

    fn store_credential(&mut self, credential: NewCredential) {
        todo!()
    }

    fn get_credentials_by_app_id(&self, app_id: String) -> Vec<Credential> {
        todo!()
    }

    fn store_secret(&mut self, secret: NewSecret) {
        todo!()
    }

    fn get_secrets_by_app_id(&self, app_id: String) -> Vec<Secret> {
        todo!()
    }
}
