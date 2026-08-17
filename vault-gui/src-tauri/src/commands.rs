use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;
use vault_core::{
    create_backup_with_rotation, decrypt_vault_from_file, encrypt_vault_to_file,
    generate_password, PasswordConfig, Vault, VaultEntry,
};

use crate::state::AppState;

/// Résultat d'une opération
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> CommandResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// Entrée sérialisable pour le frontend
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerializableEntry {
    pub id: String,
    pub service_name: String,
    pub username: String,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<&VaultEntry> for SerializableEntry {
    fn from(entry: &VaultEntry) -> Self {
        Self {
            id: entry.id.to_string(),
            service_name: entry.service_name.clone(),
            username: entry.username.clone(),
            password: entry.password.clone(),
            url: entry.url.clone(),
            notes: entry.notes.clone(),
            tags: entry.tags.clone(),
            created_at: entry.created_at,
            updated_at: entry.updated_at,
        }
    }
}

/// Crée un nouveau coffre-fort
#[tauri::command]
pub fn create_vault(
    path: String,
    master_password: String,
    state: State<AppState>,
) -> CommandResult<String> {
    let path_buf = PathBuf::from(&path);

    // Convertir en chemin absolu si nécessaire
    let abs_path = if path_buf.is_absolute() {
        path_buf
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(path_buf),
            Err(e) => return CommandResult::err(format!("Impossible de déterminer le répertoire courant: {}", e)),
        }
    };

    if abs_path.exists() {
        return CommandResult::err("Un coffre existe déjà à cet emplacement".to_string());
    }

    let vault = Vault::new();

    match encrypt_vault_to_file(&vault, &abs_path, &master_password) {
        Ok(_) => {
            let path_str = abs_path.to_string_lossy().to_string();
            *state.vault.lock().unwrap() = Some(vault);
            *state.vault_path.lock().unwrap() = Some(path_str.clone());
            *state.master_password.lock().unwrap() = Some(master_password);
            CommandResult::ok(path_str)
        }
        Err(e) => CommandResult::err(format!("Erreur lors de la création: {}", e)),
    }
}

/// Ouvre un coffre-fort existant
#[tauri::command]
pub fn open_vault(
    path: String,
    master_password: String,
    state: State<AppState>,
) -> CommandResult<Vec<SerializableEntry>> {
    let path_buf = PathBuf::from(&path);

    // Convertir en chemin absolu si nécessaire
    let abs_path = if path_buf.is_absolute() {
        path_buf
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(path_buf),
            Err(e) => return CommandResult::err(format!("Impossible de déterminer le répertoire courant: {}", e)),
        }
    };

    if !abs_path.exists() {
        return CommandResult::err("Le fichier de coffre n'existe pas".to_string());
    }

    match decrypt_vault_from_file(&abs_path, &master_password) {
        Ok(vault) => {
            let entries: Vec<SerializableEntry> = vault
                .list_entries()
                .iter()
                .map(SerializableEntry::from)
                .collect();

            let path_str = abs_path.to_string_lossy().to_string();
            *state.vault.lock().unwrap() = Some(vault);
            *state.vault_path.lock().unwrap() = Some(path_str);
            *state.master_password.lock().unwrap() = Some(master_password);

            CommandResult::ok(entries)
        }
        Err(_) => CommandResult::err("Master password incorrect ou fichier corrompu".to_string()),
    }
}

/// Verrouille le coffre-fort
#[tauri::command]
pub fn lock_vault(state: State<AppState>) -> CommandResult<()> {
    state.lock();
    CommandResult::ok(())
}

/// Vérifie si le coffre est verrouillé
#[tauri::command]
pub fn is_locked(state: State<AppState>) -> CommandResult<bool> {
    CommandResult::ok(!state.is_unlocked())
}

/// Liste toutes les entrées du coffre
#[tauri::command]
pub fn list_entries(state: State<AppState>) -> CommandResult<Vec<SerializableEntry>> {
    let vault_lock = state.vault.lock().unwrap();

    match vault_lock.as_ref() {
        Some(vault) => {
            let entries: Vec<SerializableEntry> = vault
                .list_entries()
                .iter()
                .map(SerializableEntry::from)
                .collect();
            CommandResult::ok(entries)
        }
        None => CommandResult::err("Le coffre est verrouillé".to_string()),
    }
}

/// Ajoute une nouvelle entrée
#[tauri::command]
pub fn add_entry(
    service_name: String,
    username: String,
    password: String,
    url: Option<String>,
    notes: Option<String>,
    tags: Vec<String>,
    state: State<AppState>,
) -> CommandResult<SerializableEntry> {
    let mut vault_lock = state.vault.lock().unwrap();
    let path_lock = state.vault_path.lock().unwrap();
    let password_lock = state.master_password.lock().unwrap();

    match (vault_lock.as_mut(), path_lock.as_ref(), password_lock.as_ref()) {
        (Some(vault), Some(path), Some(master_password)) => {
            let entry = VaultEntry::new(service_name, username, password, url, notes, tags);
            let serializable = SerializableEntry::from(&entry);
            vault.add_entry(entry);

            // Créer un backup avant la sauvegarde
            let path_buf = PathBuf::from(path);
            create_backup_with_rotation(&path_buf, 5).ok();

            // Sauvegarder le coffre
            match encrypt_vault_to_file(vault, &path_buf, master_password) {
                Ok(_) => CommandResult::ok(serializable),
                Err(e) => CommandResult::err(format!("Erreur lors de la sauvegarde: {}", e)),
            }
        }
        _ => CommandResult::err("Le coffre est verrouillé".to_string()),
    }
}

/// Met à jour une entrée existante
#[tauri::command]
pub fn update_entry(
    id: String,
    service_name: Option<String>,
    username: Option<String>,
    password: Option<String>,
    url: Option<String>,
    notes: Option<String>,
    tags: Option<Vec<String>>,
    state: State<AppState>,
) -> CommandResult<SerializableEntry> {
    let mut vault_lock = state.vault.lock().unwrap();
    let path_lock = state.vault_path.lock().unwrap();
    let password_lock = state.master_password.lock().unwrap();

    match (vault_lock.as_mut(), path_lock.as_ref(), password_lock.as_ref()) {
        (Some(vault), Some(path), Some(master_password)) => {
            let uuid = match uuid::Uuid::parse_str(&id) {
                Ok(u) => u,
                Err(_) => return CommandResult::err("ID invalide".to_string()),
            };

            match vault.get_entry_mut(&uuid) {
                Some(entry) => {
                    if let Some(s) = service_name {
                        entry.service_name = s;
                    }
                    if let Some(u) = username {
                        entry.username = u;
                    }
                    if let Some(p) = password {
                        entry.password = p;
                    }
                    if let Some(u) = url {
                        entry.url = Some(u);
                    }
                    if let Some(n) = notes {
                        entry.notes = Some(n);
                    }
                    if let Some(t) = tags {
                        entry.tags = t;
                    }

                    entry.updated_at = chrono::Utc::now().timestamp();

                    let serializable = SerializableEntry::from(&*entry);

                    // Créer un backup avant la sauvegarde
                    let path_buf = PathBuf::from(path);
                    create_backup_with_rotation(&path_buf, 5).ok();

                    // Sauvegarder le coffre
                    match encrypt_vault_to_file(vault, &path_buf, master_password) {
                        Ok(_) => CommandResult::ok(serializable),
                        Err(e) => CommandResult::err(format!("Erreur lors de la sauvegarde: {}", e)),
                    }
                }
                None => CommandResult::err("Entrée non trouvée".to_string()),
            }
        }
        _ => CommandResult::err("Le coffre est verrouillé".to_string()),
    }
}

/// Supprime une entrée
#[tauri::command]
pub fn delete_entry(id: String, state: State<AppState>) -> CommandResult<()> {
    let mut vault_lock = state.vault.lock().unwrap();
    let path_lock = state.vault_path.lock().unwrap();
    let password_lock = state.master_password.lock().unwrap();

    match (vault_lock.as_mut(), path_lock.as_ref(), password_lock.as_ref()) {
        (Some(vault), Some(path), Some(master_password)) => {
            let uuid = match uuid::Uuid::parse_str(&id) {
                Ok(u) => u,
                Err(_) => return CommandResult::err("ID invalide".to_string()),
            };

            match vault.remove_entry(&uuid) {
                Some(_) => {
                    // Créer un backup avant la sauvegarde
                    let path_buf = PathBuf::from(path);
                    create_backup_with_rotation(&path_buf, 5).ok();

                    // Sauvegarder le coffre
                    match encrypt_vault_to_file(vault, &path_buf, master_password) {
                        Ok(_) => CommandResult::ok(()),
                        Err(e) => CommandResult::err(format!("Erreur lors de la sauvegarde: {}", e)),
                    }
                }
                None => CommandResult::err("Entrée non trouvée".to_string()),
            }
        }
        _ => CommandResult::err("Le coffre est verrouillé".to_string()),
    }
}

/// Recherche des entrées
#[tauri::command]
pub fn search_entries(
    query: String,
    state: State<AppState>,
) -> CommandResult<Vec<SerializableEntry>> {
    let vault_lock = state.vault.lock().unwrap();

    match vault_lock.as_ref() {
        Some(vault) => {
            let results: Vec<SerializableEntry> = vault
                .search(&query)
                .iter()
                .map(|&e| SerializableEntry::from(e))
                .collect();
            CommandResult::ok(results)
        }
        None => CommandResult::err("Le coffre est verrouillé".to_string()),
    }
}

/// Génère un mot de passe
#[tauri::command]
pub fn generate_password_cmd(
    length: usize,
    use_uppercase: bool,
    use_lowercase: bool,
    use_digits: bool,
    use_symbols: bool,
    exclude_ambiguous: bool,
) -> CommandResult<String> {
    let config = PasswordConfig {
        length,
        use_uppercase,
        use_lowercase,
        use_digits,
        use_symbols,
        exclude_ambiguous,
    };

    match generate_password(&config) {
        Ok(password) => CommandResult::ok(password),
        Err(e) => CommandResult::err(e),
    }
}

/// Crée un backup manuel
#[tauri::command]
pub fn create_backup(state: State<AppState>) -> CommandResult<()> {
    let path_lock = state.vault_path.lock().unwrap();

    match path_lock.as_ref() {
        Some(path) => {
            let path_buf = PathBuf::from(path);
            // Convertir en chemin absolu si nécessaire
            let abs_path = if path_buf.is_absolute() {
                path_buf
            } else {
                match std::env::current_dir() {
                    Ok(cwd) => cwd.join(path_buf),
                    Err(_) => path_buf,
                }
            };

            match create_backup_with_rotation(&abs_path, 10) {
                Ok(_) => CommandResult::ok(()),
                Err(e) => CommandResult::err(format!("Erreur lors du backup: {}", e)),
            }
        }
        None => CommandResult::err("Aucun coffre n'est ouvert".to_string()),
    }
}
