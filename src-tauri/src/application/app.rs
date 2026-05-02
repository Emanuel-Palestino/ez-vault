use tauri::async_runtime::Mutex;
use tauri::Manager;

use crate::application::entities::EnvironmentCreate;
use crate::domain::traits::env_repository::EnvironmentRepository;
use crate::errors::VaultError;
use crate::security::session::VaultSession;
use crate::services::TursoStorage;

pub struct VaultApp<R: EnvironmentRepository> {
    version: &'static str,
    pub storage: Option<TursoStorage>,
    env_storage: Option<R>,
    pub session: Option<VaultSession>,
}

impl<R: EnvironmentRepository> VaultApp<R> {
    pub fn empty() -> Self {
        VaultApp {
            version: "0.1.0",
            storage: None,
            session: None,
            env_storage: None,
        }
    }

    pub fn version(&self) -> &str {
        self.version
    }

    pub fn store_environment(&mut self, env: EnvironmentCreate) -> Result<(), VaultError> {
        // TODO: check if env_storage is set, if not return error
        // TODO: check if session is active, if not return error
        // TODO: convert EnvironmentCreate to Environment and call env_storage.store_environment
        Ok(())
    }

    /// Returns a reference to the active session, touching last_activity.
    /// Clears and returns Locked if the session has timed out.
    pub fn require_session(&mut self) -> Result<&VaultSession, VaultError> {
        // Check expiry first (separate borrow so we can reassign self.session)
        if let Some(ref s) = self.session {
            if !s.is_active() {
                self.session = None;
                return Err(VaultError::Locked("session timed out".into()));
            }
        }
        match self.session.as_mut() {
            None => Err(VaultError::Locked("no active session".into())),
            Some(session) => {
                session.touch();
                Ok(session)
            }
        }
    }

    /// Clears the session, zeroing vault_key via ZeroizeOnDrop.
    pub fn lock(&mut self) {
        self.session = None;
    }

    pub fn set_session(&mut self, session: VaultSession) {
        self.session = Some(session);
    }

    pub fn set_storage(&mut self, storage: TursoStorage) {
        self.storage = Some(storage);
    }

    /// Returns a mutable reference to storage or an error if not connected.
    pub fn require_storage(&mut self) -> Result<&mut TursoStorage, VaultError> {
        self.storage
            .as_mut()
            .ok_or_else(|| VaultError::Internal("storage not connected".into()))
    }
}

pub fn main_tauri_setup(
) -> impl Fn(&mut tauri::App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    |app| {
        app.manage(Mutex::new(VaultApp::empty()));
        Ok(())
    }
}
