use super::app::VaultApp;
use crate::types::*;
use std::sync::Mutex;

#[tauri::command]
pub fn command_check(state: tauri::State<Mutex<VaultApp>>) {
    let vault_state = state.lock().unwrap();
    vault_state.check();
}

#[tauri::command]
pub fn command_create_environment(
    state: tauri::State<Mutex<VaultApp>>,
    environment: NewEnvironment,
) {
    let mut vault_state = state.lock().unwrap();
    vault_state.storage.store_environment(environment);
}

#[tauri::command]
pub fn command_get_environments(state: tauri::State<Mutex<VaultApp>>) -> Vec<Environment> {
    let vault_state = state.lock().unwrap();
    let environments = vault_state.storage.get_environments();
    environments.into()
}

#[tauri::command]
pub fn command_create_app(state: tauri::State<Mutex<VaultApp>>, app: NewApp) {
    let mut vault_state = state.lock().unwrap();
    vault_state.storage.store_app(app);
}

#[tauri::command]
pub fn command_get_apps(state: tauri::State<Mutex<VaultApp>>) -> Vec<App> {
    let vault_state = state.lock().unwrap();
    let apps = vault_state.storage.get_apps();
    apps.into()
}

#[tauri::command]
pub fn command_create_port(state: tauri::State<Mutex<VaultApp>>, port: NewPort) {
    let mut vault_state = state.lock().unwrap();
    vault_state.storage.store_port(port);
}

#[tauri::command]
pub fn command_get_ports(state: tauri::State<Mutex<VaultApp>>) -> Vec<Port> {
    let vault_state = state.lock().unwrap();
    let ports = vault_state.storage.get_ports();
    ports.into()
}

#[tauri::command]
pub fn command_get_ports_by_app_id(
    state: tauri::State<Mutex<VaultApp>>,
    app_id: String,
) -> Vec<Port> {
    let vault_state = state.lock().unwrap();
    let ports = vault_state.storage.get_ports_by_app_id(app_id);
    ports.into()
}

#[tauri::command]
pub fn command_create_credential(state: tauri::State<Mutex<VaultApp>>, credential: NewCredential) {
    let mut vault_state = state.lock().unwrap();
    vault_state.storage.store_credential(credential);
}

#[tauri::command]
pub fn command_get_credentials_by_app_id(
    state: tauri::State<Mutex<VaultApp>>,
    app_id: String,
) -> Vec<Credential> {
    let vault_state = state.lock().unwrap();
    let credentials = vault_state.storage.get_credentials_by_app_id(app_id);
    credentials.into()
}

#[tauri::command]
pub fn command_create_secret(state: tauri::State<Mutex<VaultApp>>, secret: NewSecret) {
    let mut vault_state = state.lock().unwrap();
    vault_state.storage.store_secret(secret);
}

#[tauri::command]
pub fn command_get_secrets_by_app_id(
    state: tauri::State<Mutex<VaultApp>>,
    app_id: String,
) -> Vec<Secret> {
    let vault_state = state.lock().unwrap();
    let secrets = vault_state.storage.get_secrets_by_app_id(app_id);
    secrets.into()
}
