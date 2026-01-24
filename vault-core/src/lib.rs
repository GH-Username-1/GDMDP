pub mod crypto;
pub mod error;
pub mod file_format;
pub mod password_generator;
pub mod vault;

// Ré-exporter les types principaux pour faciliter l'utilisation
pub use error::{Result, VaultError};
pub use file_format::{
    create_backup, create_backup_with_rotation, decrypt_vault_from_file, encrypt_vault_to_file,
};
pub use password_generator::{generate_default_password, generate_password, PasswordConfig};
pub use vault::{SecretString, Vault, VaultEntry};
