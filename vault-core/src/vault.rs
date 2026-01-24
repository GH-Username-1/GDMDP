use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Représente une entrée dans le coffre-fort
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultEntry {
    pub id: Uuid,
    pub service_name: String,
    pub username: String,
    #[serde(serialize_with = "serialize_secret", deserialize_with = "deserialize_secret")]
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

// Fonctions personnalisées pour la sérialisation des mots de passe
fn serialize_secret<S>(password: &String, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(password)
}

fn deserialize_secret<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    String::deserialize(deserializer)
}

impl VaultEntry {
    pub fn new(
        service_name: String,
        username: String,
        password: String,
        url: Option<String>,
        notes: Option<String>,
        tags: Vec<String>,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: Uuid::new_v4(),
            service_name,
            username,
            password,
            url,
            notes,
            tags,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Le coffre-fort contenant toutes les entrées
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    pub entries: Vec<VaultEntry>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Vault {
    pub fn new() -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            entries: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_entry(&mut self, entry: VaultEntry) {
        self.entries.push(entry);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn remove_entry(&mut self, id: &Uuid) -> Option<VaultEntry> {
        if let Some(pos) = self.entries.iter().position(|e| &e.id == id) {
            self.updated_at = chrono::Utc::now().timestamp();
            Some(self.entries.remove(pos))
        } else {
            None
        }
    }

    pub fn get_entry(&self, id: &Uuid) -> Option<&VaultEntry> {
        self.entries.iter().find(|e| &e.id == id)
    }

    pub fn get_entry_mut(&mut self, id: &Uuid) -> Option<&mut VaultEntry> {
        self.entries.iter_mut().find(|e| &e.id == id)
    }

    pub fn list_entries(&self) -> &[VaultEntry] {
        &self.entries
    }

    pub fn search(&self, query: &str) -> Vec<&VaultEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| {
                e.service_name.to_lowercase().contains(&query_lower)
                    || e.username.to_lowercase().contains(&query_lower)
                    || e.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
}

impl Default for Vault {
    fn default() -> Self {
        Self::new()
    }
}

/// Wrapper pour effacer les secrets de la mémoire
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecretString(pub String);

impl SecretString {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for SecretString {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}
