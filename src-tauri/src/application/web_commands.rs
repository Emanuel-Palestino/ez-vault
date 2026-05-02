use std::path::PathBuf;

use tauri::async_runtime::Mutex;
use tauri::Manager;
use zeroize::Zeroizing;

use super::app::VaultApp;
use super::storage_builder::StorageBuilder;
use crate::application::entities::EnvironmentCreate;
use crate::errors::VaultError;
use crate::interfaces::IStorage;
use crate::security::{crypto, invite, session::VaultSession, stronghold};
use crate::types::*;

// --- Setup / Session commands ---

#[tauri::command]
pub async fn command_check_setup_status(
    state: tauri::State<'_, Mutex<VaultApp>>,
    app_handle: tauri::AppHandle,
) -> Result<SetupStatus, VaultError> {
    let app_data_dir = app_data_dir(&app_handle)?;
    let needs_setup = !stronghold::credentials_exist(&app_data_dir);
    let is_locked = state.lock().await.session.is_none();
    Ok(SetupStatus {
        needs_setup,
        is_locked,
    })
}

#[tauri::command]
pub async fn command_create_vault(
    state: tauri::State<'_, Mutex<VaultApp>>,
    app_handle: tauri::AppHandle,
    email: String,
    turso_url: String,
    turso_token: String,
    master_password: String,
) -> Result<(), VaultError> {
    let app_data_dir = app_data_dir(&app_handle)?;
    let replica_path = app_data_dir.join("vault.db").to_string_lossy().to_string();

    let mut storage = StorageBuilder::new()
        .with_local_path(replica_path)
        .with_remote_url(turso_url.clone())
        .with_auth_token(turso_token.clone())
        .build_turso_storage()
        .await?;

    storage.init().await?;

    let vault_key = crypto::generate_vault_key();
    let user_salt = crypto::generate_salt();
    let user_key = crypto::derive_user_key(&master_password, &user_salt)?;
    let encrypted_vault_key = crypto::encrypt(&user_key, vault_key.as_ref())?;

    storage
        .create_vault_user(&email, &hex::encode(user_salt), &encrypted_vault_key)
        .await?;

    stronghold::save_credentials(
        &app_data_dir,
        &master_password,
        &stronghold::Credentials {
            turso_url,
            turso_token,
            user_email: email.clone(),
        },
    )?;

    let session = VaultSession::new(*vault_key, email);
    let mut vault = state.lock().await;
    vault.set_storage(storage);
    vault.set_session(session);

    Ok(())
}

#[tauri::command]
pub async fn command_join_vault(
    state: tauri::State<'_, Mutex<VaultApp>>,
    app_handle: tauri::AppHandle,
    email: String,
    turso_url: String,
    turso_token: String,
    master_password: String,
    invite_code: String,
) -> Result<(), VaultError> {
    let app_data_dir = app_data_dir(&app_handle)?;
    let replica_path = app_data_dir.join("vault.db").to_string_lossy().to_string();

    let mut storage = StorageBuilder::new()
        .with_local_path(replica_path)
        .with_remote_url(turso_url.clone())
        .with_auth_token(turso_token.clone())
        .build_turso_storage()
        .await?;

    storage.init().await?;

    let code_hash = invite::hash_invite_code(&invite_code);
    let invite_row = storage.consume_invite(&email, &code_hash).await?;

    let invite_salt_vec = hex::decode(&invite_row.salt)
        .map_err(|_| VaultError::Internal("malformed invite salt".into()))?;
    let invite_salt: [u8; crypto::SALT_LEN] = invite_salt_vec
        .try_into()
        .map_err(|_| VaultError::Internal("invalid invite salt length".into()))?;

    let invite_key = crypto::derive_user_key(&invite_code, &invite_salt)?;
    let vault_key_bytes = crypto::decrypt(&invite_key, &invite_row.encrypted_vault_key)?;

    if vault_key_bytes.len() != 32 {
        return Err(VaultError::Crypto("unexpected vault key size".into()));
    }
    let mut vault_key_arr = [0u8; 32];
    vault_key_arr.copy_from_slice(&vault_key_bytes);
    let vault_key = Zeroizing::new(vault_key_arr);

    let user_salt = crypto::generate_salt();
    let user_key = crypto::derive_user_key(&master_password, &user_salt)?;
    let encrypted_vault_key = crypto::encrypt(&user_key, vault_key.as_ref())?;

    storage
        .create_vault_user(&email, &hex::encode(user_salt), &encrypted_vault_key)
        .await?;

    stronghold::save_credentials(
        &app_data_dir,
        &master_password,
        &stronghold::Credentials {
            turso_url,
            turso_token,
            user_email: email.clone(),
        },
    )?;

    let session = VaultSession::new(*vault_key, email);
    let mut vault = state.lock().await;
    vault.set_storage(storage);
    vault.set_session(session);

    Ok(())
}

#[tauri::command]
pub async fn command_unlock(
    state: tauri::State<'_, Mutex<VaultApp>>,
    app_handle: tauri::AppHandle,
    master_password: String,
) -> Result<(), VaultError> {
    let app_data_dir = app_data_dir(&app_handle)?;

    let creds = stronghold::load_credentials(&app_data_dir, &master_password)?;

    let replica_path = app_data_dir.join("vault.db").to_string_lossy().to_string();
    let storage = StorageBuilder::new()
        .with_local_path(replica_path)
        .with_remote_url(creds.turso_url)
        .with_auth_token(creds.turso_token)
        .build_turso_storage()
        .await?;

    storage.init().await?;

    let user_row = storage.get_vault_user(&creds.user_email).await?;

    let salt_vec = hex::decode(&user_row.salt)
        .map_err(|_| VaultError::Internal("malformed user salt".into()))?;
    let salt: [u8; crypto::SALT_LEN] = salt_vec
        .try_into()
        .map_err(|_| VaultError::Internal("invalid user salt length".into()))?;

    let user_key = crypto::derive_user_key(&master_password, &salt)?;
    let vault_key_bytes = crypto::decrypt(&user_key, &user_row.encrypted_vault_key)
        .map_err(|_| VaultError::Auth("wrong master password".into()))?;

    if vault_key_bytes.len() != 32 {
        return Err(VaultError::Crypto("unexpected vault key size".into()));
    }
    let mut vault_key_arr = [0u8; 32];
    vault_key_arr.copy_from_slice(&vault_key_bytes);

    let session = VaultSession::new(vault_key_arr, creds.user_email);
    let mut vault = state.lock().await;
    vault.set_storage(storage);
    vault.set_session(session);

    Ok(())
}

#[tauri::command]
pub async fn command_lock(state: tauri::State<'_, Mutex<VaultApp>>) -> Result<(), VaultError> {
    state.lock().await.lock();
    Ok(())
}

#[tauri::command]
pub async fn command_get_current_user(
    state: tauri::State<'_, Mutex<VaultApp>>,
) -> Result<Option<String>, VaultError> {
    let mut vault = state.lock().await;
    match vault.require_session() {
        Ok(session) => Ok(Some(session.user_email.clone())),
        Err(VaultError::Locked(_)) => Ok(None),
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn command_invite_user(
    state: tauri::State<'_, Mutex<VaultApp>>,
    invitee_email: String,
) -> Result<String, VaultError> {
    let mut vault = state.lock().await;
    let vault_key: Zeroizing<[u8; 32]> = Zeroizing::new(vault.require_session()?.vault_key);

    let code = invite::generate_invite_code();
    let invite_salt = crypto::generate_salt();
    let invite_key = crypto::derive_user_key(&code, &invite_salt)?;
    let encrypted_vault_key = crypto::encrypt(&invite_key, vault_key.as_ref())?;
    let code_hash = invite::hash_invite_code(&code);
    let expires_at_ts = chrono::Utc::now().timestamp() + 86400;

    vault
        .require_storage()?
        .create_invite(
            &invitee_email,
            &code_hash,
            &hex::encode(invite_salt),
            &encrypted_vault_key,
            expires_at_ts,
        )
        .await?;

    Ok(invite::format_invite_code(&code))
}

// --- Existing CRUD commands (with session guard + field encryption) ---

#[tauri::command]
pub async fn command_get_version(
    state: tauri::State<'_, Mutex<VaultApp>>,
) -> Result<String, VaultError> {
    Ok(state.lock().await.version().to_string())
}

#[tauri::command]
pub async fn command_create_environment(
    state: tauri::State<'_, Mutex<VaultApp>>,
    environment: EnvironmentCreate,
) -> Result<(), VaultError> {
    let mut vault = state.lock().await;
    let _ = vault.require_session()?;
    vault
        .require_storage()?
        .store_environment(environment)
        .await
}

#[tauri::command]
pub async fn command_get_environments(
    state: tauri::State<'_, Mutex<VaultApp>>,
) -> Result<Vec<Environment>, VaultError> {
    let mut vault = state.lock().await;
    let _ = vault.require_session()?;
    vault.require_storage()?.get_environments().await
}

#[tauri::command]
pub async fn command_delete_environment(
    state: tauri::State<'_, Mutex<VaultApp>>,
    id: String,
) -> Result<(), VaultError> {
    let mut vault = state.lock().await;
    let _ = vault.require_session()?;
    vault.require_storage()?.delete_environment(id).await
}

#[tauri::command]
pub async fn command_create_app(
    state: tauri::State<'_, Mutex<VaultApp>>,
    app: AppCreate,
) -> Result<(), VaultError> {
    let mut vault = state.lock().await;
    let _ = vault.require_session()?;
    vault.require_storage()?.store_app(app).await
}

#[tauri::command]
pub async fn command_get_apps(
    state: tauri::State<'_, Mutex<VaultApp>>,
) -> Result<Vec<App>, VaultError> {
    let mut vault = state.lock().await;
    let _ = vault.require_session()?;
    vault.require_storage()?.get_apps().await
}

#[tauri::command]
pub async fn command_delete_app(
    state: tauri::State<'_, Mutex<VaultApp>>,
    id: String,
) -> Result<(), VaultError> {
    let mut vault = state.lock().await;
    let _ = vault.require_session()?;
    vault.require_storage()?.delete_app(id).await
}

#[tauri::command]
pub async fn command_create_credential(
    state: tauri::State<'_, Mutex<VaultApp>>,
    mut credential: CredentialCreate,
) -> Result<(), VaultError> {
    let mut vault = state.lock().await;
    let vault_key: Zeroizing<[u8; 32]> = Zeroizing::new(vault.require_session()?.vault_key);

    credential.password = credential
        .password
        .as_deref()
        .map(|p| crypto::encrypt(&vault_key, p.as_bytes()))
        .transpose()
        .map_err(VaultError::from)?;

    vault.require_storage()?.store_credential(credential).await
}

#[tauri::command]
pub async fn command_get_credentials_by_app_id(
    state: tauri::State<'_, Mutex<VaultApp>>,
    app_id: String,
) -> Result<Vec<Credential>, VaultError> {
    let mut vault = state.lock().await;
    let vault_key: Zeroizing<[u8; 32]> = Zeroizing::new(vault.require_session()?.vault_key);

    let mut credentials = vault
        .require_storage()?
        .get_credentials_by_app_id(app_id)
        .await?;

    for cred in &mut credentials {
        if let Some(enc_password) = &cred.password {
            let plaintext = crypto::decrypt(&vault_key, enc_password)?;
            cred.password = Some(
                String::from_utf8(plaintext.to_vec())
                    .map_err(|e| VaultError::Crypto(e.to_string()))?,
            );
        }
    }

    Ok(credentials)
}

#[tauri::command]
pub async fn command_delete_credential(
    state: tauri::State<'_, Mutex<VaultApp>>,
    id: String,
) -> Result<(), VaultError> {
    let mut vault = state.lock().await;
    let _ = vault.require_session()?;
    vault.require_storage()?.delete_credential(id).await
}

#[tauri::command]
pub async fn command_create_secret(
    state: tauri::State<'_, Mutex<VaultApp>>,
    mut secret: SecretCreate,
) -> Result<(), VaultError> {
    let mut vault = state.lock().await;
    let vault_key: Zeroizing<[u8; 32]> = Zeroizing::new(vault.require_session()?.vault_key);

    secret.value = crypto::encrypt(&vault_key, secret.value.as_bytes())?;

    vault.require_storage()?.store_secret(secret).await
}

#[tauri::command]
pub async fn command_get_secrets_by_app_id(
    state: tauri::State<'_, Mutex<VaultApp>>,
    app_id: String,
) -> Result<Vec<Secret>, VaultError> {
    let mut vault = state.lock().await;
    let vault_key: Zeroizing<[u8; 32]> = Zeroizing::new(vault.require_session()?.vault_key);

    let mut secrets = vault
        .require_storage()?
        .get_secrets_by_app_id(app_id)
        .await?;

    for secret in &mut secrets {
        let plaintext = crypto::decrypt(&vault_key, &secret.value)?;
        secret.value =
            String::from_utf8(plaintext.to_vec()).map_err(|e| VaultError::Crypto(e.to_string()))?;
    }

    Ok(secrets)
}

#[tauri::command]
pub async fn command_delete_secret(
    state: tauri::State<'_, Mutex<VaultApp>>,
    id: String,
) -> Result<(), VaultError> {
    let mut vault = state.lock().await;
    let _ = vault.require_session()?;
    vault.require_storage()?.delete_secret(id).await
}

#[tauri::command]
pub async fn command_create_certificate(
    state: tauri::State<'_, Mutex<VaultApp>>,
    mut certificate: CertificateCreate,
) -> Result<(), VaultError> {
    let mut vault = state.lock().await;
    let vault_key: Zeroizing<[u8; 32]> = Zeroizing::new(vault.require_session()?.vault_key);

    certificate.value = crypto::encrypt(&vault_key, certificate.value.as_bytes())?;

    vault
        .require_storage()?
        .store_certificate(certificate)
        .await
}

#[tauri::command]
pub async fn command_get_certificates_by_environment_id(
    state: tauri::State<'_, Mutex<VaultApp>>,
    environment_id: String,
) -> Result<Vec<Certificate>, VaultError> {
    let mut vault = state.lock().await;
    let vault_key: Zeroizing<[u8; 32]> = Zeroizing::new(vault.require_session()?.vault_key);

    let mut certificates = vault
        .require_storage()?
        .get_certificates_by_environment_id(environment_id)
        .await?;

    for cert in &mut certificates {
        let plaintext = crypto::decrypt(&vault_key, &cert.value)?;
        cert.value =
            String::from_utf8(plaintext.to_vec()).map_err(|e| VaultError::Crypto(e.to_string()))?;
    }

    Ok(certificates)
}

#[tauri::command]
pub async fn command_delete_certificate(
    state: tauri::State<'_, Mutex<VaultApp>>,
    id: String,
) -> Result<(), VaultError> {
    let mut vault = state.lock().await;
    let _ = vault.require_session()?;
    vault.require_storage()?.delete_certificate(id).await
}

// --- Helpers ---

fn app_data_dir(app_handle: &tauri::AppHandle) -> Result<PathBuf, VaultError> {
    let dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| VaultError::Internal(e.to_string()))?;
    std::fs::create_dir_all(&dir).map_err(|e| VaultError::Internal(e.to_string()))?;
    Ok(dir)
}
