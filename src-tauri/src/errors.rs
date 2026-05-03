use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum VaultError {
    Database(String),
    NotFound(String),
    Internal(String),
    NotConfigured(String),
}

impl std::fmt::Display for VaultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VaultError::Database(m) => write!(f, "Database error: {m}"),
            VaultError::NotFound(m) => write!(f, "Not found: {m}"),
            VaultError::Internal(m) => write!(f, "Internal error: {m}"),
            VaultError::NotConfigured(m) => write!(f, "Not configured: {m}"),
        }
    }
}

impl std::error::Error for VaultError {}

impl From<turso::Error> for VaultError {
    fn from(e: turso::Error) -> Self {
        VaultError::Database(e.to_string())
    }
}
