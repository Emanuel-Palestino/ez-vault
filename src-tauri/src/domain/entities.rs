use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Environment {
    pub id: String,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
    pub name: String,
    pub note: String,
    pub deleted: bool,
}
