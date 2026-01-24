use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use zeroize::Zeroizing;

use crate::crypto::{self, Argon2Params};
use crate::error::{Result, VaultError};
use crate::vault::Vault;

/// Magic bytes pour identifier un fichier de coffre-fort
const MAGIC: &[u8; 8] = b"RUSTPW01";

/// Version du format de fichier
const FORMAT_VERSION: u16 = 1;

/// En-tête du fichier de coffre-fort
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultFileHeader {
    /// Version du format de fichier
    pub version: u16,
    /// Paramètres Argon2
    pub argon2_memory_kib: u32,
    pub argon2_iterations: u32,
    pub argon2_parallelism: u32,
    /// Salt pour la dérivation de clé
    pub salt: Vec<u8>,
    /// Nonce pour AES-GCM
    pub nonce: [u8; 12],
}

impl VaultFileHeader {
    pub fn new(params: Argon2Params) -> Self {
        Self {
            version: FORMAT_VERSION,
            argon2_memory_kib: params.memory_cost_kib,
            argon2_iterations: params.iterations,
            argon2_parallelism: params.parallelism,
            salt: crypto::generate_salt(),
            nonce: crypto::generate_nonce(),
        }
    }

    pub fn get_argon2_params(&self) -> Argon2Params {
        Argon2Params {
            memory_cost_kib: self.argon2_memory_kib,
            iterations: self.argon2_iterations,
            parallelism: self.argon2_parallelism,
        }
    }
}

/// Structure complète du fichier de coffre-fort
#[derive(Debug, Serialize, Deserialize)]
struct VaultFile {
    header: VaultFileHeader,
    ciphertext: Vec<u8>,
}

/// Chiffre et sauvegarde un coffre-fort dans un fichier
pub fn encrypt_vault_to_file<P: AsRef<Path>>(
    vault: &Vault,
    path: P,
    master_password: &str,
) -> Result<()> {
    // Créer l'en-tête avec les paramètres par défaut
    let header = VaultFileHeader::new(Argon2Params::default());

    // Sérialiser le coffre en JSON
    let plaintext = serde_json::to_vec(vault)?;

    // Dériver la clé depuis le master password
    let key = crypto::derive_key(master_password, &header.salt, header.get_argon2_params())?;

    // Chiffrer les données
    let ciphertext = crypto::encrypt_data(&key, &header.nonce, &plaintext)?;

    // Créer la structure du fichier
    let vault_file = VaultFile { header, ciphertext };

    // Sérialiser l'ensemble (en-tête + ciphertext) en JSON
    let file_content = serde_json::to_vec(&vault_file)?;

    // Créer le fichier avec les magic bytes au début
    let mut file = File::create(path)?;
    file.write_all(MAGIC)?;
    file.write_all(&file_content)?;

    Ok(())
}

/// Déchiffre et charge un coffre-fort depuis un fichier
pub fn decrypt_vault_from_file<P: AsRef<Path>>(
    path: P,
    master_password: &str,
) -> Result<Vault> {
    // Lire le fichier
    let mut file = File::open(&path).map_err(|_| VaultError::FileNotFound)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    // Vérifier les magic bytes
    if content.len() < MAGIC.len() || &content[..MAGIC.len()] != MAGIC {
        return Err(VaultError::InvalidFormat);
    }

    // Désérialiser le fichier (sans les magic bytes)
    let vault_file: VaultFile = serde_json::from_slice(&content[MAGIC.len()..])?;

    // Vérifier la version
    if vault_file.header.version != FORMAT_VERSION {
        return Err(VaultError::InvalidFormat);
    }

    // Dériver la clé depuis le master password
    let key = crypto::derive_key(
        master_password,
        &vault_file.header.salt,
        vault_file.header.get_argon2_params(),
    )?;

    // Déchiffrer les données
    let plaintext = crypto::decrypt_data(&key, &vault_file.header.nonce, &vault_file.ciphertext)?;

    // Désérialiser le coffre
    let vault: Vault = serde_json::from_slice(&plaintext)?;

    Ok(vault)
}

/// Crée un fichier de sauvegarde
pub fn create_backup<P: AsRef<Path>>(vault_path: P) -> Result<()> {
    let vault_path = vault_path.as_ref();
    if !vault_path.exists() {
        return Err(VaultError::FileNotFound);
    }

    let backup_path = vault_path.with_extension("bak");
    fs::copy(vault_path, backup_path)?;

    Ok(())
}

/// Crée un backup avec rotation (garde les N derniers backups)
pub fn create_backup_with_rotation<P: AsRef<Path>>(vault_path: P, keep_count: usize) -> Result<()> {
    let vault_path = vault_path.as_ref();
    if !vault_path.exists() {
        return Err(VaultError::FileNotFound);
    }

    // Générer un nom de backup avec timestamp
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let backup_name = format!(
        "{}.{}.bak",
        vault_path.file_stem().and_then(|s| s.to_str()).unwrap_or("vault"),
        timestamp
    );

    let backup_path = vault_path.with_file_name(&backup_name);
    fs::copy(vault_path, &backup_path)?;

    // Nettoyer les anciens backups
    cleanup_old_backups(vault_path, keep_count)?;

    Ok(())
}

/// Nettoie les anciens backups en ne gardant que les N plus récents
fn cleanup_old_backups<P: AsRef<Path>>(vault_path: P, keep_count: usize) -> Result<()> {
    let vault_path = vault_path.as_ref();
    let parent_dir = vault_path.parent().unwrap_or(Path::new("."));
    let base_name = vault_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("vault");

    // Lister tous les fichiers de backup
    let mut backups: Vec<_> = fs::read_dir(parent_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();
            name.starts_with(base_name) && name.ends_with(".bak")
        })
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            let modified = metadata.modified().ok()?;
            Some((entry.path(), modified))
        })
        .collect();

    // Trier par date de modification (plus récent en premier)
    backups.sort_by(|a, b| b.1.cmp(&a.1));

    // Supprimer les backups excédentaires
    for (path, _) in backups.iter().skip(keep_count) {
        fs::remove_file(path).ok();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::VaultEntry;

    #[test]
    fn test_encrypt_decrypt_vault() {
        let mut vault = Vault::new();
        vault.add_entry(VaultEntry::new(
            "GitHub".to_string(),
            "user@example.com".to_string(),
            "password123".to_string(),
            Some("https://github.com".to_string()),
            None,
            vec!["dev".to_string()],
        ));

        let temp_file = "test_vault.dat";
        let password = "master_password";

        // Chiffrer et sauvegarder
        encrypt_vault_to_file(&vault, temp_file, password).unwrap();

        // Déchiffrer et charger
        let loaded_vault = decrypt_vault_from_file(temp_file, password).unwrap();

        assert_eq!(vault.entries.len(), loaded_vault.entries.len());
        assert_eq!(
            vault.entries[0].service_name,
            loaded_vault.entries[0].service_name
        );

        // Nettoyer
        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_wrong_password() {
        let vault = Vault::new();
        let temp_file = "test_vault_wrong_pwd.dat";
        let password = "correct_password";

        encrypt_vault_to_file(&vault, temp_file, password).unwrap();

        let result = decrypt_vault_from_file(temp_file, "wrong_password");
        assert!(result.is_err());

        std::fs::remove_file(temp_file).ok();
    }
}
