use crate::interfaces::IStorage;
use crate::types::*;

pub struct TursoStorage {
    pub conn: libsql::Connection,
}

impl IStorage for TursoStorage {

    /* async fn test(&self) -> Result<(), Box<dyn error::Error>> {
        println!("TursoStorage test");
        let mut rows = self.conn.query("SELECT * FROM environments", ()).await?;
        while let Some(row) = rows.next().await? {
            let id: i64 = row.get(0)?;
            let name: String = row.get(1)?;
            let note: String = row.get(2)?;
            println!("id: {}, name: {}, note: {}", id, name, note);
        }

        Ok(())
    } */

    fn store_environment(&mut self, environment: NewEnvironment) {
        todo!()
    }

    fn get_environments(&self) -> Vec<Environment> {
        todo!()
    }

    fn store_app(&mut self, app: NewApp) {
        todo!()
    }

    async fn get_apps(&self) -> Vec<App> {
        println!("TursoStorage get_apps");
        let mut rows = self.conn.query("SELECT * FROM apps", ()).await.unwrap();
        let mut apps = Vec::new();

        while let Some(row) = rows.next().await.unwrap() {
            let id: i64 = row.get(0).unwrap();
            let name: String = row.get(1).unwrap();
            let url: String = row.get(2).unwrap();
            let note: String = row.get(3).unwrap();
            let bounded_context: String = row.get(4).unwrap();
            let created_at_ts: i64 = row.get(5).unwrap();
            let updated_at_ts: i64 = row.get(6).unwrap();
            apps.push(App {
                id: id.to_string(),
                name,
                url,
                note,
                bounded_context,
                created_at_ts,
                updated_at_ts,
                environments: Vec::new(),
                labels: Vec::new(),
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
