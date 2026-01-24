use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{rand_core::RngCore, SaltString},
    Argon2, PasswordHasher,
};
use rand::RngCore as _;
use zeroize::Zeroizing;

use crate::error::{Result, VaultError};

/// Paramètres pour Argon2id
#[derive(Debug, Clone, Copy)]
pub struct Argon2Params {
    pub memory_cost_kib: u32,  // En KiB
    pub iterations: u32,
    pub parallelism: u32,
}

impl Default for Argon2Params {
    fn default() -> Self {
        Self {
            memory_cost_kib: 65536, // 64 MiB
            iterations: 3,
            parallelism: 4,
        }
    }
}

/// Génère un salt aléatoire pour Argon2
pub fn generate_salt() -> Vec<u8> {
    let salt = SaltString::generate(&mut OsRng);
    salt.as_str().as_bytes().to_vec()
}

/// Génère un nonce aléatoire pour AES-GCM (12 bytes)
pub fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}

/// Dérive une clé de chiffrement à partir du master password en utilisant Argon2id
pub fn derive_key(
    password: &str,
    salt: &[u8],
    params: Argon2Params,
) -> Result<Zeroizing<Vec<u8>>> {
    // Configuration des paramètres Argon2
    let argon2_params = argon2::Params::new(
        params.memory_cost_kib,
        params.iterations,
        params.parallelism,
        Some(32), // 256 bits pour AES-256
    )
    .map_err(|e| VaultError::CryptoError(format!("Failed to create Argon2 params: {}", e)))?;

    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2_params,
    );

    // Créer le salt au format requis
    let salt_string = SaltString::encode_b64(salt)
        .map_err(|e| VaultError::CryptoError(format!("Invalid salt: {}", e)))?;

    // Dériver la clé
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt_string)
        .map_err(|e| VaultError::CryptoError(format!("Key derivation failed: {}", e)))?;

    let hash = password_hash
        .hash
        .ok_or_else(|| VaultError::CryptoError("No hash produced".to_string()))?;

    Ok(Zeroizing::new(hash.as_bytes().to_vec()))
}

/// Chiffre des données avec AES-256-GCM
pub fn encrypt_data(key: &[u8], nonce: &[u8; 12], plaintext: &[u8]) -> Result<Vec<u8>> {
    if key.len() != 32 {
        return Err(VaultError::CryptoError(
            "Key must be 32 bytes".to_string(),
        ));
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| VaultError::CryptoError(format!("Invalid key: {}", e)))?;

    let nonce_obj = Nonce::from_slice(nonce);

    let ciphertext = cipher
        .encrypt(nonce_obj, plaintext)
        .map_err(|e| VaultError::CryptoError(format!("Encryption failed: {}", e)))?;

    Ok(ciphertext)
}

/// Déchiffre des données avec AES-256-GCM
pub fn decrypt_data(key: &[u8], nonce: &[u8; 12], ciphertext: &[u8]) -> Result<Vec<u8>> {
    if key.len() != 32 {
        return Err(VaultError::CryptoError(
            "Key must be 32 bytes".to_string(),
        ));
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| VaultError::CryptoError(format!("Invalid key: {}", e)))?;

    let nonce_obj = Nonce::from_slice(nonce);

    let plaintext = cipher
        .decrypt(nonce_obj, ciphertext)
        .map_err(|_| VaultError::InvalidMasterPassword)?; // Échec du déchiffrement = mauvais mot de passe

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_derivation() {
        let password = "test_password";
        let salt = generate_salt();
        let params = Argon2Params::default();

        let key = derive_key(password, &salt, params).unwrap();
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_encryption_decryption() {
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        let plaintext = b"Hello, World!";

        let ciphertext = encrypt_data(&key, &nonce, plaintext).unwrap();
        let decrypted = decrypt_data(&key, &nonce, &ciphertext).unwrap();

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }
}
