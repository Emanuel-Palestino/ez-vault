pub struct VaultApp {
    pub version: &'static str,
}

impl VaultApp {
    pub fn new(version: &'static str) -> Self {
        VaultApp { version }
    }

    pub fn check(&self) {
        println!("Vault app running in version {}", self.version);
    }
}
