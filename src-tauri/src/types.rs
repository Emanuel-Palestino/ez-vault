use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct NewEnvironment {
    pub name: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Environment {
    pub id: String,
    pub name: String,
    pub note: Option<String>,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
}

#[derive(Debug)]
struct App {
    pub id: String,
    pub name: String,
    pub url: String,
    pub note: Option<String>,
    pub environments: Vec<Environment>,
    pub labels: Vec<String>,
    pub bounded_context: String,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
}

#[derive(Debug)]
struct Port {
    pub id: String,
    pub app: App,
    pub value: u16,
    pub note: Option<String>,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
}

#[derive(Debug)]
struct Credential {
    pub id: String,
    pub app: App,
    pub username: String,
    pub password: String,
    pub note: Option<String>,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
}

#[derive(Debug)]
struct Secret {
    pub id: String,
    pub app: App,
    pub key: String,
    pub value: String,
    pub note: Option<String>,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
}
