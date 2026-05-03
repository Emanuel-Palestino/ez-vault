use crate::domain::entities::Certificate;
use crate::domain::traits::certificate_repository::CertificateRepository;
use crate::errors::VaultError;

pub struct TursoCertificateRepo {
    conn: turso::Connection,
}

impl TursoCertificateRepo {
    pub fn new(conn: turso::Connection) -> Self {
        Self { conn }
    }

    pub async fn init(&self) -> Result<(), VaultError> {
        self.conn
            .execute_batch(
                r#"
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
                "#,
            )
            .await
            .map_err(VaultError::from)?;

        Ok(())
    }
}

impl CertificateRepository for TursoCertificateRepo {
    async fn store_certificate(&self, data: Certificate) -> Result<(), VaultError> {
        self.conn
            .execute(
                "INSERT INTO certificates (id, name, file_name, file_extension, value, environment_id, note, created_at_ts, updated_at_ts, deleted) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0)",
                (data.id.clone(), data.name, data.file_name, data.file_extension, data.value, data.environment_id, data.note, data.created_at_ts, data.updated_at_ts),
            )
            .await
            .map_err(VaultError::from)?;

        for label in data.labels {
            self.conn
                .execute(
                    "INSERT INTO certificate_labels (certificate_id, label) VALUES (?, ?)",
                    (data.id.clone(), label),
                )
                .await
                .map_err(VaultError::from)?;
        }

        Ok(())
    }

    async fn get_certificates_by_environment_id(&self, environment_id: String) -> Result<Vec<Certificate>, VaultError> {
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

    async fn soft_delete_certificate(&self, id: String) -> Result<(), VaultError> {
        let now = chrono::Utc::now().timestamp_millis();
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
