use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use argon2::{Algorithm, Argon2, Params, Version};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use rand::RngCore;
use zeroize::Zeroizing;

pub const KEY_LEN: usize = 32;
pub const SALT_LEN: usize = 32;

#[derive(Debug)]
pub enum CryptoError {
    Encrypt,
    Decrypt,
    Decode(String),
    Kdf(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::Encrypt => write!(f, "encryption failed"),
            CryptoError::Decrypt => write!(f, "decryption failed"),
            CryptoError::Decode(m) => write!(f, "decode error: {m}"),
            CryptoError::Kdf(m) => write!(f, "key derivation error: {m}"),
        }
    }
}

impl std::error::Error for CryptoError {}

/// Derives a 32-byte key from a password using Argon2id.
/// Params: 64 MB memory, 3 iterations, 4 parallelism.
pub fn derive_user_key(
    password: &str,
    salt: &[u8; SALT_LEN],
) -> Result<Zeroizing<[u8; KEY_LEN]>, CryptoError> {
    let params =
        Params::new(65536, 3, 4, Some(KEY_LEN)).map_err(|e| CryptoError::Kdf(e.to_string()))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key = Zeroizing::new([0u8; KEY_LEN]);
    argon2
        .hash_password_into(password.as_bytes(), salt, key.as_mut())
        .map_err(|e| CryptoError::Kdf(e.to_string()))?;

    Ok(key)
}

/// Encrypts plaintext with AES-256-GCM.
/// Returns base64(nonce || ciphertext_with_tag).
pub fn encrypt(key: &[u8; KEY_LEN], plaintext: &[u8]) -> Result<String, CryptoError> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| CryptoError::Encrypt)?;

    let mut blob = nonce.to_vec();
    blob.extend_from_slice(&ciphertext);

    Ok(B64.encode(blob))
}

/// Decrypts a base64(nonce || ciphertext_with_tag) blob.
/// Returns Zeroizing<Vec<u8>> to auto-clear plaintext on drop.
pub fn decrypt(key: &[u8; KEY_LEN], encoded: &str) -> Result<Zeroizing<Vec<u8>>, CryptoError> {
    let blob = B64
        .decode(encoded)
        .map_err(|e| CryptoError::Decode(e.to_string()))?;

    if blob.len() < 12 {
        return Err(CryptoError::Decode("blob too short".into()));
    }

    let (nonce_bytes, ciphertext) = blob.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::Decrypt)?;

    Ok(Zeroizing::new(plaintext))
}

/// Generates a cryptographically random 32-byte vault key.
pub fn generate_vault_key() -> Zeroizing<[u8; KEY_LEN]> {
    let mut key = Zeroizing::new([0u8; KEY_LEN]);
    OsRng.fill_bytes(key.as_mut());
    key
}

/// Generates a cryptographically random 32-byte salt.
pub fn generate_salt() -> [u8; SALT_LEN] {
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    salt
}
