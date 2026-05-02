use std::time::Instant;
use zeroize::ZeroizeOnDrop;

pub const AUTO_LOCK_SECS: u64 = 30 * 60;

#[derive(ZeroizeOnDrop)]
pub struct VaultSession {
    pub vault_key: [u8; 32],
    pub user_email: String,
    #[zeroize(skip)]
    pub last_activity: Instant,
}

impl VaultSession {
    pub fn new(vault_key: [u8; 32], user_email: String) -> Self {
        VaultSession {
            vault_key,
            user_email,
            last_activity: Instant::now(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.last_activity.elapsed().as_secs() < AUTO_LOCK_SECS
    }

    pub fn touch(&mut self) {
        self.last_activity = Instant::now();
    }
}
