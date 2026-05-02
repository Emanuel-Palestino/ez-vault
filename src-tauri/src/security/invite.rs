use base32::Alphabet;
use rand::RngCore;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

/// Generates an 8-character uppercase base32 invite code (~2^40 entropy).
/// Displayed as XXXX-XXXX but stored/validated without the dash.
pub fn generate_invite_code() -> String {
    let mut bytes = [0u8; 5]; // 5 bytes = 40 bits → 8 base32 chars
    rand::thread_rng().fill_bytes(&mut bytes);
    let encoded = base32::encode(Alphabet::RFC4648 { padding: false }, &bytes);
    // base32 of 5 bytes is always exactly 8 chars
    encoded[..8].to_uppercase()
}

/// Formats a raw 8-char code as XXXX-XXXX for display.
pub fn format_invite_code(code: &str) -> String {
    let normalized = normalize(code);
    format!("{}-{}", &normalized[..4], &normalized[4..])
}

/// SHA-256 hex digest of the normalized code. Only this is stored in the DB.
pub fn hash_invite_code(code: &str) -> String {
    let normalized = normalize(code);
    let digest = Sha256::digest(normalized.as_bytes());
    hex::encode(digest)
}

/// Constant-time comparison of a candidate code against a stored hash.
#[allow(dead_code)]
pub fn verify_invite_code(candidate: &str, stored_hash: &str) -> bool {
    let candidate_hash = hash_invite_code(candidate);
    let a = candidate_hash.as_bytes();
    let b = stored_hash.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    a.ct_eq(b).into()
}

fn normalize(code: &str) -> String {
    code.replace('-', "").to_uppercase()
}
