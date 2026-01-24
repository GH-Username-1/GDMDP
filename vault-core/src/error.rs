use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("Cryptographic error: {0}")]
    CryptoError(String),

    #[error("Invalid master password")]
    InvalidMasterPassword,

    #[error("Vault file not found")]
    FileNotFound,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid vault format")]
    InvalidFormat,

    #[error("Entry not found: {0}")]
    EntryNotFound(String),
}

pub type Result<T> = std::result::Result<T, VaultError>;
