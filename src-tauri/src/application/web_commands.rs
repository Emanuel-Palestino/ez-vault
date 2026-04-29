use super::app::VaultApp;
use crate::{interfaces::IStorage, types::*};
use tauri::async_runtime::Mutex as AsyncMutex;
use std::sync::Mutex;

#[tauri::command]
pub fn command_check(state: tauri::State<Mutex<VaultApp>>) {
    let vault_state = state.lock().unwrap();
    vault_state.check();
}

#[tauri::command]
pub async fn command_create_environment(
    state: tauri::State<'_, AsyncMutex<VaultApp>>,
    environment: EnvironmentCreate,
) -> Result<(), ()> {
    let mut vault_state = state.lock().await;
    vault_state.storage.store_environment(environment).await;
    Ok(())
}

#[tauri::command]
pub async fn command_get_environments(state: tauri::State<'_, AsyncMutex<VaultApp>>) -> Result<Vec<Environment>, ()> {
    let vault_state = state.lock().await;
    let environments = vault_state.storage.get_environments().await;
    Ok(environments.into())
}

#[tauri::command]
pub async fn command_create_app(state: tauri::State<'_, AsyncMutex<VaultApp>>, app: AppCreate) -> Result<(), ()> {
    let mut vault_state = state.lock().await;
    vault_state.storage.store_app(app).await;
    Ok(())
}

#[tauri::command]
pub async fn command_get_apps(state: tauri::State<'_, AsyncMutex<VaultApp>>) -> Result<Vec<App>, ()> {
    let vault_state = state.lock().await;
    let apps = vault_state.storage.get_apps().await;
    Ok(apps.into())
}

#[tauri::command]
pub async fn command_create_credential(state: tauri::State<'_, AsyncMutex<VaultApp>>, credential: CredentialCreate) -> Result<(), ()> {
    let mut vault_state = state.lock().await;
    vault_state.storage.store_credential(credential).await;
    Ok(())
}

#[tauri::command]
pub async fn command_get_credentials_by_app_id(
    state: tauri::State<'_, AsyncMutex<VaultApp>>,
    app_id: String,
) -> Result<Vec<Credential>, ()> {
    let vault_state = state.lock().await;
    let credentials = vault_state.storage.get_credentials_by_app_id(app_id).await;
    Ok(credentials.into())
}

#[tauri::command]
pub async fn command_create_secret(state: tauri::State<'_, AsyncMutex<VaultApp>>, secret: SecretCreate) -> Result<(), ()> {
    let mut vault_state = state.lock().await;
    vault_state.storage.store_secret(secret).await;
    Ok(())
}

#[tauri::command]
pub async fn command_get_secrets_by_app_id(
    state: tauri::State<'_, AsyncMutex<VaultApp>>,
    app_id: String,
) -> Result<Vec<Secret>, ()> {
    let vault_state = state.lock().await;
    let secrets = vault_state.storage.get_secrets_by_app_id(app_id).await;
    Ok(secrets.into())
}

#[tauri::command]
pub async fn command_create_certificate(
    state: tauri::State<'_, AsyncMutex<VaultApp>>,
    certificate: CertificateCreate,
) -> Result<(), ()> {
    let mut vault_state = state.lock().await;
    vault_state.storage.store_certificate(certificate).await;
    Ok(())
}

#[tauri::command]
pub async fn command_get_certificates_by_environment_id(
    state: tauri::State<'_, AsyncMutex<VaultApp>>,
    environment_id: String,
) -> Result<Vec<Certificate>, ()> {
    let vault_state = state.lock().await;
    let certificates = vault_state
        .storage
        .get_certificates_by_environment_id(environment_id)
        .await;
    Ok(certificates.into())
}
