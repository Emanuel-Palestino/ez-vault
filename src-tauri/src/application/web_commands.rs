use super::app::VaultApp;
use crate::types::{Environment, NewEnvironment};
use std::sync::Mutex;

#[tauri::command]
pub fn ez_vault_check(state: tauri::State<Mutex<VaultApp>>) {
    let vault_state = state.lock().unwrap();
    vault_state.check();
}

#[tauri::command]
pub fn ez_vault_create_environment(
    state: tauri::State<Mutex<VaultApp>>,
    environment: NewEnvironment,
) {
    let mut vault_state = state.lock().unwrap();
    vault_state.storage.store_environment(environment);
}

#[tauri::command]
pub fn ez_vault_get_environments(state: tauri::State<Mutex<VaultApp>>) -> Vec<Environment> {
    let vault_state = state.lock().unwrap();
    let environments = vault_state.storage.get_environments();
    environments.into()
}
