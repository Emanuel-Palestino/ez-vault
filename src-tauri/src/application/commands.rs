use crate::application::VaultApp;

#[tauri::command]
pub fn ez_vault_check(state: tauri::State<VaultApp>) {
    state.check();
}