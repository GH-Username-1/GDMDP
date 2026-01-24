# Changelog

Toutes les modifications notables de ce projet sont documentées dans ce fichier.

## [Phase 5] - 2025-12-07

### Ajouté
- **Interface graphique complète** : Application desktop moderne et fonctionnelle
- **Composants React** :
  - `LoginScreen.tsx` : Écran de connexion avec modes "Ouvrir" et "Créer"
  - `EntryCard.tsx` : Carte d'affichage avec masquage du mot de passe
  - `EntryForm.tsx` : Modal de formulaire pour add/edit avec générateur intégré
  - `SearchBar.tsx` : Barre de recherche en temps réel avec bouton clear
- **Hooks personnalisés** :
  - `useAutoLock.ts` : Auto-lock après 5 minutes d'inactivité
- **Fonctionnalités UI** :
  - Générateur de mots de passe intégré dans le formulaire
  - Recherche en temps réel
  - Copie dans le presse-papiers (Tauri clipboard API)
  - Notifications toast pour feedback utilisateur
  - Masquage/affichage des mots de passe
  - Grille responsive d'entrées
  - Auto-lock avec détection d'activité utilisateur
  - Toggle auto-lock (bouton ⏱️/⏸️)
- **Design moderne** :
  - Thème sombre complet avec CSS variables
  - Animations et transitions fluides
  - Scrollbar personnalisée
  - Responsive design (mobile + desktop)
  - Hover effects et focus states
  - Modal avec overlay semi-transparent

### Amélioré
- **App.tsx** : Orchestration complète de l'application
  - Gestion de l'état global (entries, filteredEntries, searchQuery)
  - Intégration de tous les composants
  - Handlers CRUD complets
  - Auto-refresh après modifications
- **VaultService** : Gestion d'erreur cohérente pour toutes les commandes
- **Expérience utilisateur** :
  - Formulaires avec validation
  - États de chargement (loading states)
  - Messages d'erreur clairs
  - Confirmations de suppression
  - Feedback immédiat pour toutes les actions

### Sécurité
- Auto-lock configurable après inactivité (5 min par défaut)
- Masquage par défaut des mots de passe (●●●●●)
- Copie sécurisée dans le presse-papiers
- Verrouillage manuel instantané
- Effacement de l'état en mémoire au lock

### UX/UI
- Empty states informatifs (aucune entrée, aucun résultat)
- Animations d'entrée pour notifications (slideIn)
- Grille adaptative avec auto-fill
- Icônes émoji pour actions (📋 copier, ✏️ éditer, 🗑️ supprimer)
- Tags visuels pour organisation
- Métadonnées avec dates de création/modification

## [Phase 4] - 2025-12-07

### Ajouté
- **Projet `vault-gui`** : Application desktop avec Tauri + React + TypeScript
- **Backend Tauri** :
  - Module `state.rs` : Gestion d'état thread-safe avec Mutex
  - Module `commands.rs` : 11 commandes exposées au frontend
  - Gestion de session (coffre ouvert/verrouillé)
  - Conservation du master password en mémoire pendant la session
- **Commandes Tauri** :
  - `create_vault` : Création de coffre
  - `open_vault` : Ouverture avec déchiffrement
  - `lock_vault` : Verrouillage et effacement de la session
  - `is_locked` : Vérification de l'état
  - `list_entries` : Liste des entrées
  - `add_entry` : Ajout avec backup automatique
  - `update_entry` : Modification avec backup
  - `delete_entry` : Suppression avec backup
  - `search_entries` : Recherche
  - `generate_password_cmd` : Génération de mots de passe
  - `create_backup` : Backup manuel
- **Frontend React** :
  - Service `VaultService` : Wrapper TypeScript pour toutes les commandes
  - Types TypeScript complets
  - Interface basique : Login + liste des entrées
  - Styles CSS responsive
- **Configuration** :
  - `tauri.conf.json` : Configuration Tauri sécurisée
  - `vite.config.ts` : Build optimisé
  - `tsconfig.json` : TypeScript strict
  - `package.json` : Dépendances Node.js

### Architecture
- Crate Rust `vault-gui` ajouté au workspace
- Séparation claire backend (Rust) / frontend (React)
- Communication via invoke Tauri
- État partagé thread-safe

### Sécurité
- Allowlist Tauri restrictive (seulement dialogs)
- Pas d'accès filesystem direct depuis le frontend
- État thread-safe avec Mutex
- Backups automatiques avant toute modification
- Gestion d'erreurs sans exposition de données sensibles

### Performance
- Master password dérivé une seule fois par session
- Opérations suivantes rapides (password en mémoire)
- Hot-reload en développement

## [Phase 3] - 2025-12-07

### Ajouté
- **Commande `search`** : Recherche d'entrées par service, username ou tags
- **Commande `gen`** : Générateur de mots de passe avec options configurables
  - Longueur personnalisable
  - Options pour inclure/exclure maj, min, chiffres, symboles
  - Exclusion des caractères ambigus par défaut
  - Génération de plusieurs mots de passe à la fois
- **Commande `update`** : Modification d'entrées existantes
  - Mise à jour de tous les champs individuellement
  - Option `--generate` pour générer un nouveau mot de passe
  - Mise à jour des timestamps
- **Commande `delete`** : Suppression d'entrées avec confirmation
  - Confirmation par défaut (sécurité)
  - Option `--yes` pour bypass
- **Commande `backup`** : Création manuelle de backups
  - Rotation automatique des backups (garde N plus récents)
  - Nom de fichier avec timestamp
- **Backups automatiques** : Avant chaque modification (add, update, delete)
- **Option `--generate`** dans `add` : Génération automatique de mot de passe
- **Option `--tags`** dans `list` : Affichage des tags
- **Module `password_generator`** dans vault-core
- **Fonction `create_backup_with_rotation()`** dans vault-core
- **Fonction `cleanup_old_backups()`** dans vault-core

### Amélioré
- **Messages utilisateur** : Ajout d'émojis pour meilleure lisibilité
  - ✅ pour succès
  - ❌ pour erreurs
  - ⚠️ pour avertissements
  - 🔐🔑🔍📊📅 pour contexte
- **Commande `show`** : Affichage des dates de création/modification
- **Validation** : Avertissement si master password < 8 caractères
- **Affichage `list`** : Meilleur formatage avec option tags
- **Messages d'erreur** : Plus clairs et informatifs

### Sécurité
- Génération cryptographiquement sûre des mots de passe
- Backups automatiques avant modifications (prévention perte de données)
- Validation renforcée des entrées utilisateur

## [Phase 1 & 2] - 2025-12-05

### Ajouté
- **Workspace Rust** avec deux crates :
  - `vault-core` : Bibliothèque crypto et gestion du coffre
  - `vault-cli` : Interface en ligne de commande
- **Cryptographie** :
  - Dérivation de clé avec Argon2id (64 MiB, 3 iter, 4 threads)
  - Chiffrement AES-256-GCM (AEAD)
  - Génération sécurisée de salt et nonce
- **Format de fichier** :
  - Magic bytes `RUSTPW01`
  - En-tête JSON avec métadonnées
  - Ciphertext authentifié
- **Structures de données** :
  - `Vault` : Conteneur principal
  - `VaultEntry` : Entrée avec id, service, username, password, url, notes, tags
  - `SecretString` : Wrapper avec zeroize
- **Commandes CLI** :
  - `init` : Créer un coffre
  - `add` : Ajouter une entrée
  - `list` : Lister les entrées
  - `show` : Afficher les détails d'une entrée
- **Gestion des erreurs** : Types d'erreurs avec thiserror
- **Tests unitaires** :
  - Tests de dérivation de clé
  - Tests de chiffrement/déchiffrement
  - Tests end-to-end du vault
  - Tests de rejet avec mauvais password
- **Documentation** :
  - README.md
  - QUICKSTART.md
  - ARCHITECTURE.md
  - LIVRAISON_PHASE1_2.md
  - test_example.md

### Sécurité
- Aucun mot de passe en clair sur disque
- Master password jamais stocké
- Secrets effacés de la mémoire avec zeroize
- AEAD avec GCM pour authentification + chiffrement
- Pas de logs de secrets

## Comparaison des versions

| Fonctionnalité | Phase 1-2 | Phase 3 | Phase 5 |
|----------------|-----------|---------|---------|
| Commandes CLI | 4 | 9 | 9 |
| Interface graphique | ❌ | ❌ | ✅ |
| Recherche | ❌ | ✅ (CLI) | ✅ (GUI temps réel) |
| Générateur de mots de passe | ❌ | ✅ (CLI) | ✅ (GUI intégré) |
| Modification d'entrées | ❌ | ✅ | ✅ |
| Suppression d'entrées | ❌ | ✅ | ✅ |
| Backups automatiques | ❌ | ✅ | ✅ |
| Auto-lock | ❌ | ❌ | ✅ (5 min) |
| Copie presse-papiers | ❌ | ❌ | ✅ |
| Messages améliorés | Basiques | Émojis | Notifications toast |
| Validation master password | ❌ | ✅ | ✅ |
| Affichage timestamps | ❌ | ✅ | ✅ |
| Design moderne | ❌ | ❌ | ✅ (dark theme) |

## Migration

### De Phase 2 vers Phase 3

Aucune migration nécessaire. Les coffres créés en Phase 2 sont 100% compatibles avec Phase 3.

```bash
# Compiler la nouvelle version
cargo build --release

# Utiliser avec votre coffre existant
./target/release/vault list
./target/release/vault search <terme>
./target/release/vault gen
```

## Roadmap

### Phase 6 (Future)
- Import/Export de coffres (JSON, CSV)
- Historique des modifications
- Récupération de mots de passe supprimés
- Analyse de force des mots de passe

### Phase 7 (Future)
- Support multi-coffres amélioré
- Groupes et catégories d'entrées
- Favoris et notes épinglées
- Thèmes personnalisables (light/dark/custom)

### Phase 8 (Future)
- Extension navigateur
- Auto-fill de formulaires web
- Détection automatique des sites

### Phase 9 (Future)
- Synchronisation cloud optionnelle (chiffrement E2E)
- Partage sécurisé d'entrées
- Authentification multi-facteur (2FA)

## Notes de version

### Phase 5 - Notes importantes

1. **Auto-lock** : L'auto-lock se déclenche après 5 minutes d'inactivité. Peut être désactivé temporairement via le bouton ⏱️ dans le header.

2. **Générateur intégré** : Le générateur de mots de passe est accessible directement dans le formulaire d'ajout/édition avec options configurables (longueur, types de caractères).

3. **Recherche temps réel** : La recherche s'effectue automatiquement à chaque frappe dans tous les champs (service, username, tags, notes).

4. **Notifications** : Les toast notifications s'affichent pendant 3 secondes pour confirmer les actions (copie, sauvegarde, suppression, etc.).

5. **Responsive** : L'interface s'adapte automatiquement aux écrans mobiles et tablettes.

6. **Icône** : Si l'icône est manquante lors du build, exécuter `.\vault-gui\create-icon.ps1` (Windows).

7. **Compatibilité coffres** : Les coffres créés en CLI (Phase 1-3) sont compatibles avec la GUI et vice-versa.

### Phase 3 - Notes importantes

1. **Backups automatiques** : Désormais, chaque modification (add, update, delete) crée automatiquement un backup. Par défaut, 5 backups sont conservés.

2. **Nommage des backups** : Les backups utilisent des timestamps : `vault.20231205_143022.bak`

3. **Émojis** : Les messages utilisent maintenant des émojis. Assurez-vous que votre terminal supporte UTF-8.

4. **Générateur de mots de passe** : Utilise le RNG cryptographique de Rust (`rand::thread_rng()`). Exclut par défaut les caractères ambigus (0, O, 1, l, I) pour faciliter la saisie manuelle.

5. **Compatibilité** : 100% rétrocompatible avec les coffres Phase 2.

### Phase 1-2 - Notes importantes

1. **Master password** : Aucune récupération possible si oublié. Faites des backups du fichier `.dat`.

2. **Performance** : Le temps de dérivation Argon2id (~300-500ms) est volontairement lent pour la sécurité.

3. **Format ouvert** : Le format de fichier est documenté et peut être lu par d'autres outils compatibles.
