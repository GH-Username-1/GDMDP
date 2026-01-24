use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use vault_core::Vault;

/// État de l'application
#[derive(Debug)]
pub struct AppState {
    /// Le coffre actuellement ouvert (None si verrouillé)
    pub vault: Mutex<Option<Vault>>,
    /// Le chemin du fichier de coffre
    pub vault_path: Mutex<Option<String>>,
    /// Le master password (conservé en mémoire tant que déverrouillé)
    pub master_password: Mutex<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            vault: Mutex::new(None),
            vault_path: Mutex::new(None),
            master_password: Mutex::new(None),
        }
    }

    /// Vérifie si un coffre est ouvert
    pub fn is_unlocked(&self) -> bool {
        self.vault.lock().unwrap().is_some()
    }

    /// Verrouille le coffre
    pub fn lock(&self) {
        *self.vault.lock().unwrap() = None;
        *self.master_password.lock().unwrap() = None;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Information sur l'état de verrouillage
#[derive(Debug, Serialize, Deserialize)]
pub struct LockStatus {
    pub is_locked: bool,
    pub vault_path: Option<String>,
}
