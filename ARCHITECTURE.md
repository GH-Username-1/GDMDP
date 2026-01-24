# Architecture Technique

## Vue d'ensemble

Le projet est structuré en workspace Rust avec séparation claire entre la logique métier (vault-core) et l'interface utilisateur (vault-cli).

```
┌─────────────────────────────────────────┐
│           vault-cli                     │
│  (Interface en ligne de commande)       │
│                                         │
│  - Parsing des arguments (clap)         │
│  - Saisie sécurisée du password         │
│  - Affichage des résultats              │
└──────────────┬──────────────────────────┘
               │
               │ utilise
               ▼
┌─────────────────────────────────────────┐
│           vault-core                    │
│  (Bibliothèque crypto et gestion)       │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │  vault.rs                       │   │
│  │  - Vault (conteneur)            │   │
│  │  - VaultEntry (entrée)          │   │
│  │  - SecretString (wrapper)       │   │
│  └─────────────────────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │  crypto.rs                      │   │
│  │  - derive_key() [Argon2id]      │   │
│  │  - encrypt_data() [AES-GCM]     │   │
│  │  - decrypt_data() [AES-GCM]     │   │
│  └─────────────────────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │  file_format.rs                 │   │
│  │  - encrypt_vault_to_file()      │   │
│  │  - decrypt_vault_from_file()    │   │
│  │  - VaultFileHeader              │   │
│  └─────────────────────────────────┘   │
│                                         │
│  ┌─────────────────────────────────┐   │
│  │  error.rs                       │   │
│  │  - VaultError (types d'erreurs) │   │
│  │  - Result<T>                    │   │
│  └─────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

## Flux de données

### Création d'un coffre (init)

```
User input
    ↓
master_password (String)
    ↓
generate_salt() → salt (Vec<u8>)
    ↓
derive_key(password, salt, params) → key (32 bytes)
    ↓
Vault::new() → vault (vide)
    ↓
serde_json::to_vec(&vault) → plaintext (JSON)
    ↓
generate_nonce() → nonce (12 bytes)
    ↓
encrypt_data(key, nonce, plaintext) → ciphertext
    ↓
VaultFile { header, ciphertext }
    ↓
File: [MAGIC][JSON{header, ciphertext}]
```

### Ajout d'une entrée

```
User input (service, username, password, etc.)
    ↓
VaultEntry::new(...) → entry
    ↓
decrypt_vault_from_file(path, master_password) → vault
    ↓
vault.add_entry(entry)
    ↓
encrypt_vault_to_file(&vault, path, master_password)
```

### Lecture du coffre

```
File: [MAGIC][JSON{header, ciphertext}]
    ↓
Vérification MAGIC bytes
    ↓
Désérialisation header → VaultFileHeader
    ↓
derive_key(master_password, header.salt, header.params) → key
    ↓
decrypt_data(key, header.nonce, ciphertext) → plaintext
    ↓                                              |
    |                                 Si échec: InvalidMasterPassword
    ↓
serde_json::from_slice(&plaintext) → Vault
    ↓
vault.list_entries() / vault.get_entry(id)
```

## Détails cryptographiques

### Argon2id - Dérivation de clé

**Paramètres par défaut :**
- Memory: 65536 KiB (64 MiB)
- Iterations: 3
- Parallelism: 4
- Output length: 32 bytes (256 bits)

**Flux :**
```
master_password (String)
    +
salt (aléatoire, stocké en clair)
    +
params (stockés en clair)
    ↓
  Argon2id
    ↓
encryption_key (32 bytes, en mémoire uniquement)
```

**Propriétés :**
- Résistant aux attaques par GPU/ASIC (memory-hard)
- Protection contre les attaques par canal auxiliaire
- Recommandé par OWASP pour le hashing de passwords

### AES-256-GCM - Chiffrement

**Paramètres :**
- Algorithm: AES
- Key size: 256 bits
- Mode: GCM (Galois/Counter Mode)
- Nonce: 12 bytes (96 bits), aléatoire

**Flux :**
```
plaintext (JSON du vault)
    +
key (32 bytes depuis Argon2id)
    +
nonce (12 bytes, aléatoire, stocké en clair)
    ↓
 AES-256-GCM
    ↓
ciphertext + authentication_tag
```

**Propriétés :**
- AEAD (Authenticated Encryption with Associated Data)
- Détecte toute modification du ciphertext
- Performance : ~1-5 GB/s sur CPU moderne
- Nonce unique requis pour chaque opération de chiffrement

## Format de fichier

### Structure binaire

```
┌──────────────────────────────────────────────┐
│  Magic Bytes (8 bytes)                       │
│  "RUSTPW01"                                  │
├──────────────────────────────────────────────┤
│  JSON {                                      │
│    "header": {                               │
│      "version": u16,                         │
│      "argon2_memory_kib": u32,               │
│      "argon2_iterations": u32,               │
│      "argon2_parallelism": u32,              │
│      "salt": [u8],                           │
│      "nonce": [u8; 12]                       │
│    },                                        │
│    "ciphertext": [u8]                        │
│  }                                           │
└──────────────────────────────────────────────┘
```

### Exemple (vue conceptuelle)

```json
RUSTPW01{
  "header": {
    "version": 1,
    "argon2_memory_kib": 65536,
    "argon2_iterations": 3,
    "argon2_parallelism": 4,
    "salt": [142, 78, 201, ...],  // 22+ bytes base64
    "nonce": [45, 128, 93, ...]   // 12 bytes
  },
  "ciphertext": [encrypted_data_with_auth_tag]
}
```

### Données déchiffrées (plaintext)

```json
{
  "entries": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "service_name": "GitHub",
      "username": "user@example.com",
      "password": "GitHubPass123!",
      "url": "https://github.com",
      "notes": "Compte principal",
      "tags": ["dev", "work"],
      "created_at": 1703001234,
      "updated_at": 1703001234
    }
  ],
  "created_at": 1703001234,
  "updated_at": 1703001234
}
```

## Sécurité - Mesures implémentées

### 1. Protection des secrets en mémoire

```rust
use zeroize::{Zeroize, ZeroizeOnDrop};

// Clé dérivée effacée automatiquement
let key: Zeroizing<Vec<u8>> = derive_key(...);

// Au moment où key sort du scope, la mémoire est overwritée
```

### 2. Validation de l'intégrité

```rust
// AES-GCM vérifie automatiquement l'authentication tag
let plaintext = cipher.decrypt(nonce, ciphertext)
    .map_err(|_| VaultError::InvalidMasterPassword)?;

// Si le tag ne correspond pas → erreur
// Causes possibles :
// - Mauvais master password
// - Fichier corrompu
// - Tentative de modification
```

### 3. Pas de stockage du master password

Le master password n'est **jamais** stocké. À chaque opération :
1. L'utilisateur saisit le master password
2. La clé est dérivée avec Argon2id
3. Tentative de déchiffrement
4. Si succès → bon password ; si échec → mauvais password
5. La clé dérivée est effacée de la mémoire

### 4. Gestion des erreurs

```rust
pub enum VaultError {
    CryptoError(String),        // Erreur crypto interne
    InvalidMasterPassword,      // Mauvais password ou fichier corrompu
    FileNotFound,               // Coffre inexistant
    IoError,                    // Problème lecture/écriture
    SerializationError,         // Problème JSON
    InvalidFormat,              // Format de fichier incorrect
    EntryNotFound(String),      // Entrée introuvable
}
```

Aucune information sensible dans les messages d'erreur.

## Performance

### Temps typiques (machine moderne)

- **Dérivation de clé (Argon2id)** : 200-500 ms
- **Chiffrement/déchiffrement (AES-GCM)** : < 1 ms pour un petit coffre
- **Chargement complet** : ~300-600 ms (dominé par Argon2id)

### Optimisations futures possibles

- Cache de la clé dérivée pendant une session (avec auto-lock)
- Parallélisation de certaines opérations
- Compression avant chiffrement pour gros coffres

## Dépendances clés

| Crate | Version | Usage |
|-------|---------|-------|
| aes-gcm | 0.10 | Chiffrement AES-256-GCM |
| argon2 | 0.5 | Dérivation de clé KDF |
| serde | 1.0 | Sérialisation JSON |
| zeroize | 1.7 | Effacement sécurisé de la mémoire |
| uuid | 1.6 | Identifiants uniques pour les entrées |
| clap | 4.4 | Parsing des arguments CLI |
| rpassword | 7.3 | Saisie masquée du password |

Toutes proviennent de sources fiables et maintenues activement.

## Extensibilité

### Phase 3 (prévue)

- Ajout de `password_generator.rs` dans vault-core
- Commande `search` avec recherche floue
- Rotation automatique de backups

### Phase 4-5 (GUI)

```
vault-gui/
├── src-tauri/          # Backend Rust
│   └── src/
│       ├── commands.rs # Commandes Tauri exposées au frontend
│       └── state.rs    # Gestion de l'état (coffre ouvert/verrouillé)
└── src/                # Frontend React/TypeScript
    ├── components/
    ├── pages/
    └── App.tsx
```

L'architecture actuelle (vault-core séparé) facilite l'intégration avec Tauri.
