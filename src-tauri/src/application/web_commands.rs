use super::app::VaultApp;
use crate::application::types::*;
use crate::domain::entities::{App, Certificate, Credential, Environment, Secret};
use crate::errors::VaultError;
use tauri::async_runtime::Mutex;

#[tauri::command]
pub async fn command_get_version(
    state: tauri::State<'_, Mutex<VaultApp>>,
) -> Result<String, VaultError> {
    let vault_state = state.lock().await;
    Ok(vault_state.version().to_string())
}

#[tauri::command]
pub async fn command_create_environment(
    state: tauri::State<'_, Mutex<VaultApp>>,
    environment: EnvironmentCreate,
) -> Result<(), VaultError> {
    let vault_state = state.lock().await;
    vault_state.create_environment(environment).await
}

#[tauri::command]
pub async fn command_get_environments(
    state: tauri::State<'_, Mutex<VaultApp>>,
) -> Result<Vec<Environment>, VaultError> {
    let vault_state = state.lock().await;
    vault_state.get_environments().await
}

#[tauri::command]
pub async fn command_delete_environment(
    state: tauri::State<'_, Mutex<VaultApp>>,
    id: String,
) -> Result<(), VaultError> {
    let vault_state = state.lock().await;
    vault_state.delete_environment(id).await
}

#[tauri::command]
pub async fn command_create_app(
    state: tauri::State<'_, Mutex<VaultApp>>,
    app: AppCreate,
) -> Result<(), VaultError> {
    let vault_state = state.lock().await;
    vault_state.create_app(app).await
}

#[tauri::command]
pub async fn command_get_apps(
    state: tauri::State<'_, Mutex<VaultApp>>,
) -> Result<Vec<App>, VaultError> {
    let vault_state = state.lock().await;
    vault_state.get_apps().await
}

#[tauri::command]
pub async fn command_delete_app(
    state: tauri::State<'_, Mutex<VaultApp>>,
    id: String,
) -> Result<(), VaultError> {
    let vault_state = state.lock().await;
    vault_state.delete_app(id).await
}

#[tauri::command]
pub async fn command_create_credential(
    state: tauri::State<'_, Mutex<VaultApp>>,
    credential: CredentialCreate,
) -> Result<(), VaultError> {
    let vault_state = state.lock().await;
    vault_state.create_credential(credential).await
}

#[tauri::command]
pub async fn command_get_credentials_by_app_id(
    state: tauri::State<'_, Mutex<VaultApp>>,
    app_id: String,
) -> Result<Vec<Credential>, VaultError> {
    let vault_state = state.lock().await;
    vault_state.get_credentials_by_app_id(app_id).await
}

#[tauri::command]
pub async fn command_delete_credential(
    state: tauri::State<'_, Mutex<VaultApp>>,
    id: String,
) -> Result<(), VaultError> {
    let vault_state = state.lock().await;
    vault_state.delete_credential(id).await
}

#[tauri::command]
pub async fn command_create_secret(
    state: tauri::State<'_, Mutex<VaultApp>>,
    secret: SecretCreate,
) -> Result<(), VaultError> {
    let vault_state = state.lock().await;
    vault_state.create_secret(secret).await
}

#[tauri::command]
pub async fn command_get_secrets_by_app_id(
    state: tauri::State<'_, Mutex<VaultApp>>,
    app_id: String,
) -> Result<Vec<Secret>, VaultError> {
    let vault_state = state.lock().await;
    vault_state.get_secrets_by_app_id(app_id).await
}

#[tauri::command]
pub async fn command_delete_secret(
    state: tauri::State<'_, Mutex<VaultApp>>,
    id: String,
) -> Result<(), VaultError> {
    let vault_state = state.lock().await;
    vault_state.delete_secret(id).await
}

#[tauri::command]
pub async fn command_create_certificate(
    state: tauri::State<'_, Mutex<VaultApp>>,
    certificate: CertificateCreate,
) -> Result<(), VaultError> {
    let vault_state = state.lock().await;
    vault_state.create_certificate(certificate).await
}

#[tauri::command]
pub async fn command_get_certificates_by_environment_id(
    state: tauri::State<'_, Mutex<VaultApp>>,
    environment_id: String,
) -> Result<Vec<Certificate>, VaultError> {
    let vault_state = state.lock().await;
    vault_state
        .get_certificates_by_environment_id(environment_id)
        .await
}

#[tauri::command]
pub async fn command_delete_certificate(
    state: tauri::State<'_, Mutex<VaultApp>>,
    id: String,
) -> Result<(), VaultError> {
    let vault_state = state.lock().await;
    vault_state.delete_certificate(id).await
}
