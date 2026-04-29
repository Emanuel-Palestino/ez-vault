// IMPORTANT NOTE: If changes are made to this file you must to sync the changes with the typescript entities types
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    pub id: String,
    pub created_at_ts: i64,
    pub updated_at_ts: i64,
    pub name: String,
    pub note: String,
    pub deleted: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentCreate {
    pub name: String,
    pub note: String,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppCreate {
    pub name: String,
    pub url: String,
    pub environment_id: String,
    pub labels: Vec<String>,
    pub note: String,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialCreate {
    pub app_id: String,
    pub context: String,
    pub username: String,
    pub password: Option<String>,
    pub url: Option<String>,
    pub note: String,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretCreate {
    pub app_id: String,
    pub key: String,
    pub value: String,
    pub note: String,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateCreate {
    pub name: String,
    pub file_name: String,
    pub file_extension: String,
    pub value: String,
    pub environment_id: String,
    pub labels: Vec<String>,
    pub note: String,
}
