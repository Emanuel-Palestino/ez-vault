// IMPORTANT NOTE: If changes are made to this file you must to sync the changes with the typescript entities types
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveConfigInput {
    pub database_url: String,
    pub database_token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentCreate {
    pub name: String,
    pub note: String,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretCreate {
    pub app_id: String,
    pub key: String,
    pub value: String,
    pub note: String,
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
