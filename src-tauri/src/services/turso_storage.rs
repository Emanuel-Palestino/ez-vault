use libsql::Builder;

use crate::interfaces::IStorage;

#[derive(Debug, Clone)]
pub struct TursoStorage {
    pub db: libsql::Database,
    pub conn: libsql::Connection,
}

impl TursoStorage {

    async fn test(&self) {
        println!("TursoStorage test");
        let mut rows = self.conn.execute("SELECT * FROM environments", ()).await?;
        while let Some(row) = rows.next().await? {
            let id: i64 = row.get("id")?;
            let name: String = row.get("name")?;
            let note: String = row.get("note")?;
            println!("id: {}, name: {}, note: {}", id, name, note);
        }
    }

}
