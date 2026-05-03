use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Environment {
    pub id: String,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
    pub name: String,
    pub note: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct App {
    pub id: String,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
    pub name: String,
    pub url: String,
    pub environment_id: String,
    pub labels: Vec<String>,
    pub note: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
    pub id: String,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
    pub app_id: String,
    pub context: String,
    pub username: String,
    pub password: Option<String>,
    pub url: Option<String>,
    pub note: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    pub id: String,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
    pub app_id: String,
    pub key: String,
    pub value: String,
    pub note: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Certificate {
    pub id: String,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
    pub name: String,
    pub file_name: String,
    pub file_extension: String,
    pub value: String,
    pub environment_id: String,
    pub labels: Vec<String>,
    pub note: String,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub database_url: String,
    pub database_token: String,
}
