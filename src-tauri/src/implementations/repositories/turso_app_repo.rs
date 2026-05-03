use crate::domain::entities::App;
use crate::domain::traits::app_repository::AppRepository;
use crate::errors::VaultError;

pub struct TursoAppRepo {
    conn: turso::Connection,
}

impl TursoAppRepo {
    pub fn new(conn: turso::Connection) -> Self {
        Self { conn }
    }

    pub async fn init(&self) -> Result<(), VaultError> {
        self.conn
            .execute_batch(
                r#"
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
                "#,
            )
            .await
            .map_err(VaultError::from)?;

        Ok(())
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

impl AppRepository for TursoAppRepo {
    async fn store_app(&self, data: App) -> Result<(), VaultError> {
        self.conn
            .execute(
                "INSERT INTO apps (id, name, url, environment_id, note, created_at_ts, updated_at_ts, deleted) VALUES (?, ?, ?, ?, ?, ?, ?, 0)",
                (data.id.clone(), data.name, data.url, data.environment_id, data.note, data.created_at_ts, data.updated_at_ts),
            )
            .await
            .map_err(VaultError::from)?;

        for label in data.labels {
            self.conn
                .execute(
                    "INSERT INTO app_labels (app_id, label) VALUES (?, ?)",
                    (data.id.clone(), label),
                )
                .await
                .map_err(VaultError::from)?;
        }

        Ok(())
    }

    async fn get_apps(&self) -> Result<Vec<App>, VaultError> {
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

    async fn soft_delete_app(&self, id: String) -> Result<(), VaultError> {
        let now = chrono::Utc::now().timestamp_millis();
        self.conn
            .execute(
                "UPDATE apps SET deleted = 1, updated_at_ts = ? WHERE id = ?",
                (now, id),
            )
            .await
            .map_err(VaultError::from)?;
        Ok(())
    }
}
