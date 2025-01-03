// IMPORTANT NOTE: If changes are made to this file you must to sync the changes with the typescript entities types
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NewEnvironment {
    pub name: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Environment {
    pub id: String,
    pub name: String,
    pub note: String,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
}

#[derive(Debug, Deserialize)]
pub struct NewApp {
    pub name: String,
    pub url: String,
    pub note: String,
    pub environments: Vec<String>,
    pub labels: Vec<String>,
    pub bounded_context: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct App {
    pub id: String,
    pub name: String,
    pub url: String,
    pub note: String,
    pub environments: Vec<Environment>,
    pub labels: Vec<String>,
    pub bounded_context: String,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
}

#[derive(Debug, Deserialize)]
pub struct NewPort {
    pub app_id: String,
    pub value: u16,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Port {
    pub id: String,
    pub app: App,
    pub value: u16,
    pub note: String,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
}

#[derive(Debug, Deserialize)]
pub struct NewCredential {
    pub app_id: String,
    pub username: String,
    pub password: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Credential {
    pub id: String,
    pub app: App,
    pub username: String,
    pub password: String,
    pub note: String,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
}

#[derive(Debug, Deserialize)]
pub struct NewSecret {
    pub app_id: String,
    pub key: String,
    pub value: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Secret {
    pub id: String,
    pub app: App,
    pub key: String,
    pub value: String,
    pub note: String,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
}
