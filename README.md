# Gestionnaire de Mots de Passe Local (GDMDP)

Un gestionnaire de mots de passe sécurisé écrit en Rust avec chiffrement AES-256-GCM et dérivation de clé Argon2id.

**Phase actuelle : Phase 5 - GUI Minimale (Complète)**

## Architecture

Le projet est organisé en workspace Rust avec trois crates :

- **vault-core** : Bibliothèque contenant la logique crypto et la gestion du coffre
- **vault-cli** : Interface en ligne de commande complète
- **vault-gui** : Application de bureau avec Tauri + React

## Prérequis

### Pour le CLI (vault-cli)
- Rust 1.70+ (édition 2021)
- Cargo

### Pour la GUI (vault-gui)
- Rust 1.70+ (édition 2021)
- Node.js 18+
- npm ou pnpm

Installation de Rust :
```bash
# Windows
# Téléchargez rustup depuis https://rustup.rs/

# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation

### CLI (vault-cli)

```bash
# Naviguer dans le dossier
cd GDMDP_

# Compiler le CLI
cargo build --release

# Le binaire sera disponible dans target/release/vault
```

### GUI (vault-gui)

```bash
# Naviguer dans le dossier GUI
cd vault-gui

# Installer les dépendances
npm install

# Développement
npm run tauri dev

# Build production
npm run tauri build
```

## Utilisation

### Commandes disponibles

| Commande | Description |
|----------|-------------|
| `init` | Créer un nouveau coffre-fort |
| `add` | Ajouter une entrée (avec option de génération auto) |
| `update` | Modifier une entrée existante |
| `delete` | Supprimer une entrée |
| `list` | Lister toutes les entrées |
| `search` | Rechercher des entrées |
| `show` | Afficher les détails d'une entrée |
| `gen` | Générer un mot de passe aléatoire |
| `backup` | Créer un backup manuel |

### Exemples rapides

```bash
# Initialiser un coffre
vault init

# Ajouter avec génération automatique de mot de passe
vault add --service GitHub --username user@example.com --generate

# Générer un mot de passe seul
vault gen --length 20

# Rechercher des entrées
vault search github

# Lister avec tags
vault list --tags

# Mettre à jour une entrée
vault update GitHub --username new@email.com --generate

# Supprimer une entrée
vault delete OldService --yes

# Créer un backup
vault backup --keep 10
```

### Initialiser un nouveau coffre-fort

```bash
# Créer un nouveau coffre-fort (vault.dat par défaut)
vault init

# Ou spécifier un chemin personnalisé
vault --file mon_coffre.dat init
```

### Ajouter une entrée

```bash
# Mode interactif (le mot de passe sera demandé en mode masqué)
vault add --service GitHub --username user@example.com

# Avec génération automatique de mot de passe
vault add --service Gmail --username user@gmail.com --generate

# Avec mot de passe personnalisé de 24 caractères
vault add --service Slack --username john@company.com -g --gen-length 24

# Avec toutes les options
vault add \
  --service GitHub \
  --username user@example.com \
  --url https://github.com \
  --notes "Compte principal" \
  --tags dev,work \
  --generate
```

### Lister les entrées

```bash
# Liste basique
vault list

# Liste avec tags
vault list --tags
```

### Rechercher des entrées

```bash
# Recherche dans services, usernames et tags
vault search github
vault search work
vault search @gmail.com
```

### Afficher une entrée

```bash
# Par nom de service
vault show GitHub

# Par ID
vault show 550e8400-e29b-41d4-a716-446655440000
```

### Mettre à jour une entrée

```bash
# Changer le username
vault update GitHub --username new@email.com

# Générer un nouveau mot de passe
vault update Gmail --generate

# Mise à jour multiple
vault update GitHub \
  --username admin@github.com \
  --url https://github.com/enterprise \
  --tags work,dev,important
```

### Supprimer une entrée

```bash
# Avec confirmation
vault delete GitHub

# Sans confirmation
vault delete OldService --yes
```

### Générer des mots de passe

```bash
# Mot de passe par défaut (16 caractères)
vault gen

# Longueur personnalisée
vault gen --length 32

# Sans symboles
vault gen --no-symbols

# PIN à 6 chiffres
vault gen --no-uppercase --no-lowercase --no-symbols -l 6

# Générer plusieurs mots de passe
vault gen --count 5

# Options avancées
vault gen \
  --length 24 \
  --no-symbols \
  --include-ambiguous \
  --count 3
```

### Sauvegardes

```bash
# Backup manuel (garde 10 backups par défaut)
vault backup

# Personnaliser le nombre de backups conservés
vault backup --keep 5
```

**Fonctionnement :**
- **Backups automatiques** : Créés avant chaque modification (add, update, delete), garde les 5 derniers
- **Backups manuels** : Créés avec la commande `backup`, garde les 10 derniers par défaut
- **Emplacement** : Dans le même dossier que votre coffre
- **Format** : `vault.AAAAMMJJ_HHMMSS.bak` (ex: `vault.20260131_185921.bak`)
- **Rotation** : Les plus anciens sont supprimés automatiquement selon la limite configurée

**Restaurer un backup :**
```bash
# Méthode simple : renommer le fichier .bak en .dat
cp vault.20260131_185921.bak vault.dat
```

## Sécurité

### Chiffrement

- **AES-256-GCM** : Chiffrement authentifié (AEAD)
- **Argon2id** : Dérivation de clé à partir du master password
  - Paramètres par défaut : 64 MiB de mémoire, 3 itérations, 4 threads
  - Temps de dérivation ~200-500ms sur une machine moderne

### Format de fichier

Le fichier de coffre contient :
1. Magic bytes (`RUSTPW01`)
2. Version du format
3. En-tête JSON avec :
   - Paramètres Argon2 (mémoire, itérations, parallélisme)
   - Salt (généré aléatoirement)
   - Nonce AES-GCM (12 bytes)
4. Ciphertext (données chiffrées et authentifiées)

### Bonnes pratiques

- Le master password n'est **jamais stocké**
- Les secrets en mémoire sont effacés avec `zeroize`
- Aucun mot de passe n'est affiché dans les logs
- Le fichier de coffre est protégé par AEAD (authentification + chiffrement)
- Backups automatiques avant chaque modification
- Génération cryptographiquement sûre des mots de passe

## Structure du code

```
GDMDP_/
├── vault-core/              # Bibliothèque principale
│   └── src/
│       ├── lib.rs           # API publique
│       ├── vault.rs         # Structures de données (Vault, VaultEntry)
│       ├── crypto.rs        # Fonctions cryptographiques
│       ├── file_format.rs   # Sérialisation et format de fichier
│       ├── password_generator.rs  # Générateur de mots de passe
│       └── error.rs         # Gestion des erreurs
├── vault-cli/               # Interface CLI
│   └── src/
│       └── main.rs          # Toutes les commandes clap
├── vault-gui/               # Application de bureau
│   ├── src/                 # Frontend React/TypeScript
│   │   ├── components/      # Composants React
│   │   ├── hooks/           # Hooks personnalisés
│   │   ├── services/        # Services API
│   │   ├── App.tsx          # Application principale
│   │   └── styles.css       # Styles CSS
│   └── src-tauri/           # Backend Tauri (Rust)
│       ├── src/
│       │   ├── commands.rs  # Commandes exposées
│       │   ├── state.rs     # État partagé
│       │   └── main.rs      # Point d'entrée
│       └── tauri.conf.json  # Configuration Tauri
├── Cargo.toml               # Workspace
├── README.md                # Ce fichier
├── QUICKSTART.md            # Guide de démarrage rapide
├── ARCHITECTURE.md          # Architecture technique détaillée
├── LIVRAISON_PHASE1_2.md    # Documentation Phase 1 & 2
├── LIVRAISON_PHASE3.md      # Documentation Phase 3
├── LIVRAISON_PHASE5.md      # Documentation Phase 5 (GUI)
└── test_example_phase3.md   # Scénarios de test Phase 3
```

## Tests

```bash
# Exécuter les tests
cargo test

# Tests avec sortie détaillée
cargo test -- --nocapture

# Tests d'un module spécifique
cargo test --package vault-core password_generator
```

## Fonctionnalités implémentées

### Phase 1 & 2
- ✅ Core Rust avec crypto (Argon2id + AES-256-GCM)
- ✅ Format de fichier sécurisé
- ✅ CLI basique : init, add, list, show
- ✅ Gestion des erreurs
- ✅ Tests unitaires

### Phase 3 (Actuelle)
- ✅ Commande `search` - Recherche d'entrées
- ✅ Commande `gen` - Générateur de mots de passe configurable
- ✅ Commande `update` - Modification d'entrées
- ✅ Commande `delete` - Suppression avec confirmation
- ✅ Option `--generate` dans add/update
- ✅ Backups automatiques avec rotation
- ✅ Messages améliorés avec émojis
- ✅ Validation du master password
- ✅ Affichage des timestamps

### Phase 4
- ✅ Configuration Tauri + React
- ✅ Backend Rust avec 11 commandes
- ✅ Frontend TypeScript avec services
- ✅ Structure complète du projet

### Phase 5 (Actuelle)
- ✅ Interface graphique complète
- ✅ Écran de connexion (création/ouverture)
- ✅ Dashboard avec grille d'entrées
- ✅ Formulaires CRUD complets
- ✅ Générateur de mots de passe UI
- ✅ Recherche en temps réel
- ✅ Auto-lock après 5 min d'inactivité
- ✅ Copie dans le presse-papiers
- ✅ Notifications toast
- ✅ Design moderne et responsive

### Phases futures (6+)
- ⬜ Import/Export sécurisé
- ⬜ Historique des modifications
- ⬜ Support multi-coffres amélioré
- ⬜ Extension navigateur

## Documentation complète

- **[QUICKSTART.md](QUICKSTART.md)** : Guide de démarrage rapide avec installation
- **[ARCHITECTURE.md](ARCHITECTURE.md)** : Architecture technique et détails crypto
- **[LIVRAISON_PHASE1_2.md](LIVRAISON_PHASE1_2.md)** : Documentation des Phases 1 & 2
- **[LIVRAISON_PHASE3.md](LIVRAISON_PHASE3.md)** : Documentation de la Phase 3
- **[LIVRAISON_PHASE5.md](LIVRAISON_PHASE5.md)** : Documentation de la Phase 5 (GUI)
- **[test_example_phase3.md](test_example_phase3.md)** : Scénarios de test complets

## Exemples d'utilisation avancée

### Workflow complet

```bash
# 1. Initialiser
vault init

# 2. Ajouter plusieurs entrées
vault add -s GitHub -u dev@company.com -g --gen-length 24
vault add -s Gmail -u personal@gmail.com -g -t email,personal
vault add -s Slack -u john@work.com -t work,communication --notes "Team workspace"

# 3. Rechercher et consulter
vault search work
vault show GitHub

# 4. Mettre à jour
vault update Gmail --generate  # Nouveau mot de passe
vault update Slack --notes "Main workspace" --tags work,main

# 5. Backup manuel
vault backup --keep 10

# 6. Nettoyer
vault delete OldService -y
```

### Génération de mots de passe avancée

```bash
# Mot de passe ultra-sécurisé
vault gen -l 32 --count 1

# PIN bancaire
vault gen --no-uppercase --no-lowercase --no-symbols -l 4

# Mot de passe compatible (sans symboles)
vault gen --no-symbols -l 16

# Batch de mots de passe
vault gen -l 20 --count 10
```

### Gestion multi-coffres

```bash
# Coffre personnel
vault --file personal.dat init
vault --file personal.dat add -s Gmail -u me@gmail.com -g

# Coffre professionnel
vault --file work.dat init
vault --file work.dat add -s Slack -u me@company.com -g

# Lister chaque coffre
vault --file personal.dat list
vault --file work.dat list
```

## Dépendances

### vault-core
- `aes-gcm` : Chiffrement AES-256-GCM
- `argon2` : Dérivation de clé
- `rand` : Génération aléatoire cryptographique
- `serde` + `serde_json` : Sérialisation
- `zeroize` : Effacement sécurisé
- `thiserror` : Gestion d'erreurs
- `uuid` : Identifiants uniques
- `chrono` : Timestamps

### vault-cli
- `vault-core` : Logique métier
- `clap` : Parsing CLI
- `rpassword` : Saisie masquée
- `uuid` : Parsing UUIDs
- `chrono` : Formatage dates

## Contribution

Ce projet est éducatif et à usage personnel.

## Licence

Projet éducatif - À usage personnel

## Support

Pour des questions ou signaler des bugs, consultez la documentation ou les fichiers de test.
