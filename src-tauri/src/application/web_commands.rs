use super::app::VaultApp;
use crate::{interfaces::IStorage, types::*};
use std::sync::Mutex;
use tauri::async_runtime::Mutex as AsyncMutex;

#[tauri::command]
pub fn command_check(state: tauri::State<Mutex<VaultApp>>) {
    let vault_state = state.lock().unwrap();
    vault_state.check();
}

#[tauri::command]
pub async fn command_create_environment(
    state: tauri::State<'_, AsyncMutex<VaultApp>>,
    environment: NewEnvironment,
) -> Result<(), ()> {
    let mut vault_state = state.lock().await;
    vault_state.storage.store_environment(environment).await;
    Ok(())
}

#[tauri::command]
pub async fn command_get_environments(
    state: tauri::State<'_, AsyncMutex<VaultApp>>,
) -> Result<Vec<Environment>, ()> {
    let vault_state = state.lock().await;
    let environments = vault_state.storage.get_environments().await;
    Ok(environments.into())
}

#[tauri::command]
pub async fn command_create_app(
    state: tauri::State<'_, AsyncMutex<VaultApp>>,
    app: NewApp,
) -> Result<(), ()> {
    let mut vault_state = state.lock().await;
    vault_state.storage.store_app(app).await;
    Ok(())
}

#[tauri::command]
pub async fn command_get_apps(
    state: tauri::State<'_, AsyncMutex<VaultApp>>,
) -> Result<Vec<App>, ()> {
    let vault_state = state.lock().await;
    let apps = vault_state.storage.get_apps().await;
    Ok(apps.into())
}

#[tauri::command]
pub async fn command_create_port(
    state: tauri::State<'_, AsyncMutex<VaultApp>>,
    port: NewPort,
) -> Result<(), ()> {
    let mut vault_state = state.lock().await;
    vault_state.storage.store_port(port).await;
    Ok(())
}

#[tauri::command]
pub async fn command_get_ports(
    state: tauri::State<'_, AsyncMutex<VaultApp>>,
) -> Result<Vec<Port>, ()> {
    let vault_state = state.lock().await;
    let ports = vault_state.storage.get_ports().await;
    Ok(ports.into())
}

#[tauri::command]
pub async fn command_get_ports_by_app_id(
    state: tauri::State<'_, AsyncMutex<VaultApp>>,
    app_id: String,
) -> Result<Vec<Port>, ()> {
    let vault_state = state.lock().await;
    let ports = vault_state.storage.get_ports_by_app_id(app_id).await;
    Ok(ports.into())
}

#[tauri::command]
pub async fn command_create_credential(
    state: tauri::State<'_, AsyncMutex<VaultApp>>,
    credential: NewCredential,
) -> Result<(), ()> {
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
pub async fn command_create_secret(
    state: tauri::State<'_, AsyncMutex<VaultApp>>,
    secret: NewSecret,
) -> Result<(), ()> {
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
