use super::app::VaultApp;
use crate::application::types::*;
use crate::domain::entities::{App, Certificate, Credential, Environment, Secret};
use crate::errors::VaultError;
use tauri::async_runtime::Mutex;

#[tauri::command]
pub async fn command_get_version(
    state: tauri::State<'_, Mutex<VaultApp>>,
) -> Result<String, VaultError> {
    let app = state.lock().await;
    Ok(app.version().to_string())
}

#[tauri::command]
pub async fn command_create_environment(
    state: tauri::State<'_, Mutex<VaultApp>>,
    environment: EnvironmentCreate,
) -> Result<(), VaultError> {
    let app = state.lock().await;
    app.domain.create_environment(environment.name, environment.note).await
}

#[tauri::command]
pub async fn command_get_environments(
    state: tauri::State<'_, Mutex<VaultApp>>,
) -> Result<Vec<Environment>, VaultError> {
    let app = state.lock().await;
    app.domain.get_environments().await
}

#[tauri::command]
pub async fn command_delete_environment(
    state: tauri::State<'_, Mutex<VaultApp>>,
    id: String,
) -> Result<(), VaultError> {
    let app = state.lock().await;
    app.domain.delete_environment(id).await
}

#[tauri::command]
pub async fn command_create_app(
    state: tauri::State<'_, Mutex<VaultApp>>,
    app: AppCreate,
) -> Result<(), VaultError> {
    let vault = state.lock().await;
    vault.domain
        .create_app(app.name, app.url, app.environment_id, app.labels, app.note)
        .await
}

#[tauri::command]
pub async fn command_get_apps(
    state: tauri::State<'_, Mutex<VaultApp>>,
) -> Result<Vec<App>, VaultError> {
    let app = state.lock().await;
    app.domain.get_apps().await
}

#[tauri::command]
pub async fn command_delete_app(
    state: tauri::State<'_, Mutex<VaultApp>>,
    id: String,
) -> Result<(), VaultError> {
    let app = state.lock().await;
    app.domain.delete_app(id).await
}

#[tauri::command]
pub async fn command_create_credential(
    state: tauri::State<'_, Mutex<VaultApp>>,
    credential: CredentialCreate,
) -> Result<(), VaultError> {
    let app = state.lock().await;
    app.domain
        .create_credential(
            credential.app_id,
            credential.context,
            credential.username,
            credential.password,
            credential.url,
            credential.note,
        )
        .await
}

#[tauri::command]
pub async fn command_get_credentials_by_app_id(
    state: tauri::State<'_, Mutex<VaultApp>>,
    app_id: String,
) -> Result<Vec<Credential>, VaultError> {
    let app = state.lock().await;
    app.domain.get_credentials_by_app_id(app_id).await
}

#[tauri::command]
pub async fn command_delete_credential(
    state: tauri::State<'_, Mutex<VaultApp>>,
    id: String,
) -> Result<(), VaultError> {
    let app = state.lock().await;
    app.domain.delete_credential(id).await
}

#[tauri::command]
pub async fn command_create_secret(
    state: tauri::State<'_, Mutex<VaultApp>>,
    secret: SecretCreate,
) -> Result<(), VaultError> {
    let app = state.lock().await;
    app.domain
        .create_secret(secret.app_id, secret.key, secret.value, secret.note)
        .await
}

#[tauri::command]
pub async fn command_get_secrets_by_app_id(
    state: tauri::State<'_, Mutex<VaultApp>>,
    app_id: String,
) -> Result<Vec<Secret>, VaultError> {
    let app = state.lock().await;
    app.domain.get_secrets_by_app_id(app_id).await
}

#[tauri::command]
pub async fn command_delete_secret(
    state: tauri::State<'_, Mutex<VaultApp>>,
    id: String,
) -> Result<(), VaultError> {
    let app = state.lock().await;
    app.domain.delete_secret(id).await
}

#[tauri::command]
pub async fn command_create_certificate(
    state: tauri::State<'_, Mutex<VaultApp>>,
    certificate: CertificateCreate,
) -> Result<(), VaultError> {
    let app = state.lock().await;
    app.domain
        .create_certificate(
            certificate.name,
            certificate.file_name,
            certificate.file_extension,
            certificate.value,
            certificate.environment_id,
            certificate.labels,
            certificate.note,
        )
        .await
}

#[tauri::command]
pub async fn command_get_certificates_by_environment_id(
    state: tauri::State<'_, Mutex<VaultApp>>,
    environment_id: String,
) -> Result<Vec<Certificate>, VaultError> {
    let app = state.lock().await;
    app.domain.get_certificates_by_environment_id(environment_id).await
}

#[tauri::command]
pub async fn command_delete_certificate(
    state: tauri::State<'_, Mutex<VaultApp>>,
    id: String,
) -> Result<(), VaultError> {
    let app = state.lock().await;
    app.domain.delete_certificate(id).await
}
