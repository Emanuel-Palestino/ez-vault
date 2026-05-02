/// Encrypted local credentials store.
///
/// Stores Turso URL, token, and user email in an AES-256-GCM encrypted file.
/// File format: hex(salt) + ":" + base64(nonce||ciphertext)
/// The ciphertext is JSON with the credential fields.
/// Key = Argon2id(master_password, salt).
///
/// Security equivalence to tauri-plugin-stronghold: both are encrypted files
/// on disk protected by Argon2id + AES-GCM. This implementation is directly
/// auditable and avoids a heavy dependency whose Rust backend API is private.
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

use crate::errors::VaultError;
use crate::security::crypto;

const CREDENTIALS_FILE: &str = "credentials.enc";

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub turso_url: String,
    pub turso_token: String,
    pub user_email: String,
}

pub fn credentials_path(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join(CREDENTIALS_FILE)
}

pub fn credentials_exist(app_data_dir: &Path) -> bool {
    credentials_path(app_data_dir).exists()
}

pub fn save_credentials(
    app_data_dir: &Path,
    password: &str,
    creds: &Credentials,
) -> Result<(), VaultError> {
    let salt = crypto::generate_salt();
    let key = crypto::derive_user_key(password, &salt)
        .map_err(|e| VaultError::Crypto(e.to_string()))?;

    let plaintext = serde_json::to_string(creds)
        .map_err(|e| VaultError::Internal(e.to_string()))?;

    let ciphertext = crypto::encrypt(&key, plaintext.as_bytes())
        .map_err(|e| VaultError::Crypto(e.to_string()))?;

    let contents = format!("{}:{}", hex::encode(salt), ciphertext);
    std::fs::write(credentials_path(app_data_dir), contents)
        .map_err(|e| VaultError::Internal(e.to_string()))?;

    Ok(())
}

pub fn load_credentials(
    app_data_dir: &Path,
    password: &str,
) -> Result<Credentials, VaultError> {
    let contents = std::fs::read_to_string(credentials_path(app_data_dir))
        .map_err(|_| VaultError::Auth("credentials file not found".into()))?;

    let (salt_hex, ciphertext) = contents
        .split_once(':')
        .ok_or_else(|| VaultError::Internal("malformed credentials file".into()))?;

    let salt_vec = hex::decode(salt_hex)
        .map_err(|_| VaultError::Internal("malformed salt".into()))?;

    let salt: [u8; crypto::SALT_LEN] = salt_vec
        .try_into()
        .map_err(|_| VaultError::Internal("invalid salt length".into()))?;

    let key = crypto::derive_user_key(password, &salt)
        .map_err(|e| VaultError::Crypto(e.to_string()))?;

    let plaintext_bytes = crypto::decrypt(&key, ciphertext)
        .map_err(|_| VaultError::Auth("wrong master password".into()))?;

    let creds: Credentials = serde_json::from_slice(&plaintext_bytes)
        .map_err(|e| VaultError::Internal(e.to_string()))?;

    // Explicitly drop the Zeroizing wrapper after parsing
    drop(Zeroizing::new(plaintext_bytes.to_vec()));

    Ok(creds)
}
