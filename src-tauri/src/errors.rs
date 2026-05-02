use serde::Serialize;

use crate::security::crypto::CryptoError;

#[derive(Debug, Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum VaultError {
    Database(String),
    NotFound(String),
    Internal(String),
    Locked(String),
    Crypto(String),
    Auth(String),
}

impl std::fmt::Display for VaultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VaultError::Database(m) => write!(f, "Database error: {m}"),
            VaultError::NotFound(m) => write!(f, "Not found: {m}"),
            VaultError::Internal(m) => write!(f, "Internal error: {m}"),
            VaultError::Locked(m) => write!(f, "Locked: {m}"),
            VaultError::Crypto(m) => write!(f, "Crypto error: {m}"),
            VaultError::Auth(m) => write!(f, "Auth error: {m}"),
        }
    }
}

impl std::error::Error for VaultError {}

impl From<turso::Error> for VaultError {
    fn from(e: turso::Error) -> Self {
        VaultError::Database(e.to_string())
    }
}

impl From<CryptoError> for VaultError {
    fn from(e: CryptoError) -> Self {
        VaultError::Crypto(e.to_string())
    }
}
