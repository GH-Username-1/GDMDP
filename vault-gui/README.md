# Vault GUI - Interface graphique du gestionnaire de mots de passe

Application desktop basée sur Tauri + React + TypeScript.

## Prérequis

- Rust 1.70+
- Node.js 18+
- npm ou yarn

## Installation

```bash
# Installer les dépendances
npm install
```

## Développement

```bash
# Lancer en mode développement avec hot-reload
npm run tauri dev
```

Cela lance :
- Serveur Vite sur http://localhost:5173
- Application Tauri native
- Auto-reload pour les modifications

## Build

```bash
# Build de production
npm run tauri build
```

Le binaire sera dans `src-tauri/target/release/`

## Structure

```
vault-gui/
├── src/                # Rust (backend Tauri)
│   ├── main.rs        # Point d'entrée
│   ├── state.rs       # Gestion d'état
│   └── commands.rs    # Commandes Tauri
│
├── src/                # React/TypeScript (frontend)
│   ├── App.tsx        # Composant principal
│   ├── types.ts       # Types
│   └── services/      # Services
│
├── Cargo.toml         # Config Rust
├── package.json       # Config Node
└── tauri.conf.json    # Config Tauri
```

## Commandes Tauri disponibles

- `create_vault` - Créer un nouveau coffre
- `open_vault` - Ouvrir un coffre existant
- `lock_vault` - Verrouiller le coffre
- `list_entries` - Lister les entrées
- `add_entry` - Ajouter une entrée
- `update_entry` - Modifier une entrée
- `delete_entry` - Supprimer une entrée
- `search_entries` - Rechercher des entrées
- `generate_password_cmd` - Générer un mot de passe
- `create_backup` - Créer un backup

## Utilisation

1. Lancer l'application
2. Entrer le chemin du fichier de coffre
3. Entrer le master password
4. Déverrouiller

Les entrées créées avec `vault-cli` sont compatibles.

## Phase actuelle : 4

Phase 4 : Backend Tauri complet + Interface basique

Phase 5 (à venir) : Interface utilisateur complète

## Licence

Projet éducatif - À usage personnel
