# Script de Test Manuel

Ce fichier décrit les étapes pour tester manuellement le gestionnaire de mots de passe.

## Prérequis

1. Compilez d'abord le projet :
```bash
cargo build --release
```

2. Naviguez dans le dossier du binaire ou utilisez le chemin complet

## Scénario de test complet

### Étape 1 : Initialisation

```bash
# Windows
.\target\release\vault.exe init

# Linux/macOS
./target/release/vault init
```

**Test attendu :**
- Demande du master password
- Demande de confirmation
- Message de succès : "✓ Coffre-fort créé avec succès: vault.dat"
- Fichier `vault.dat` créé dans le dossier courant

### Étape 2 : Tentative de ré-initialisation

```bash
.\target\release\vault.exe init
```

**Test attendu :**
- Message : "Un coffre-fort existe déjà à cet emplacement"

### Étape 3 : Ajouter des entrées

```bash
# Entrée 1 : GitHub
.\target\release\vault.exe add --service GitHub --username dev@example.com

# Le CLI demandera :
# - Master password: [entrez le même que lors de l'init]
# - Mot de passe pour ce service: [entrez ex: GitHubPass123!]

# Entrée 2 : Gmail avec options complètes
.\target\release\vault.exe add \
  --service Gmail \
  --username john.doe@gmail.com \
  --password "GmailSecure456!" \
  --url https://mail.google.com \
  --notes "Compte personnel principal" \
  --tags email,personal

# Entrée 3 : Slack
.\target\release\vault.exe add \
  --service Slack \
  --username john@company.com \
  --tags work,communication
```

**Tests attendus :**
- Chaque commande demande le master password
- Message de confirmation : "✓ Entrée ajoutée: [Service]"
- Le fichier vault.dat est mis à jour

### Étape 4 : Lister les entrées

```bash
.\target\release\vault.exe list
```

**Test attendu :**
```
Master password: [entrez votre master password]

ID                                    Service                        Username
----------------------------------------------------------------------------------------------------
550e8400-e29b-41d4-a716-446655440000  GitHub                         dev@example.com
660e8400-e29b-41d4-a716-446655440001  Gmail                          john.doe@gmail.com
770e8400-e29b-41d4-a716-446655440002  Slack                          john@company.com

Total: 3 entrée(s)
```

### Étape 5 : Afficher une entrée

```bash
# Par nom de service
.\target\release\vault.exe show GitHub
```

**Test attendu :**
```
Master password: [entrez votre master password]

============================================================
Service:  GitHub
Username: dev@example.com
Password: GitHubPass123!
ID:       550e8400-e29b-41d4-a716-446655440000
============================================================
```

```bash
# Afficher Gmail (avec tous les champs)
.\target\release\vault.exe show Gmail
```

**Test attendu :**
```
============================================================
Service:  Gmail
Username: john.doe@gmail.com
Password: GmailSecure456!
URL:      https://mail.google.com
Notes:    Compte personnel principal
Tags:     email, personal
ID:       660e8400-e29b-41d4-a716-446655440001
============================================================
```

### Étape 6 : Recherche avec mauvais nom

```bash
.\target\release\vault.exe show NonExistant
```

**Test attendu :**
```
Master password: [entrez votre master password]
Aucune entrée trouvée pour: NonExistant
```

### Étape 7 : Test avec mauvais master password

```bash
.\target\release\vault.exe list
# Entrez un mauvais master password
```

**Test attendu :**
```
Master password: [entrez un mauvais password]
Erreur: Invalid master password
```

### Étape 8 : Test avec fichier personnalisé

```bash
# Créer un second coffre
.\target\release\vault.exe --file work.dat init

# Ajouter une entrée
.\target\release\vault.exe --file work.dat add --service Jira --username admin

# Lister (devrait montrer seulement Jira)
.\target\release\vault.exe --file work.dat list
```

**Test attendu :**
- Deux fichiers distincts : `vault.dat` et `work.dat`
- Chaque coffre a son propre master password et ses propres entrées

## Tests de sécurité à vérifier

1. **Chiffrement** : Ouvrez `vault.dat` dans un éditeur de texte
   - Vous devriez voir les magic bytes "RUSTPW01" au début
   - Le reste doit être illisible (données chiffrées)
   - AUCUN mot de passe en clair ne doit apparaître

2. **Format JSON de l'en-tête** : Les métadonnées (salt, nonce, params) sont en JSON mais lisibles
   - C'est normal et voulu
   - Les mots de passe sont dans le ciphertext, pas dans l'en-tête

3. **Master password** :
   - N'est jamais affiché pendant la saisie (mode masqué)
   - N'est jamais stocké dans le fichier
   - Mauvais password = impossible de déchiffrer

## Nettoyage après tests

```bash
# Supprimer les fichiers de test
rm vault.dat
rm work.dat
rm *.bak  # si des backups ont été créés
```

## Tests unitaires automatiques

```bash
# Lancer les tests intégrés dans le code
cargo test

# Tests avec sortie détaillée
cargo test -- --nocapture

# Tests d'un module spécifique
cargo test --package vault-core --lib crypto
```

**Tests attendus :**
- `test_key_derivation` : Vérifie que la dérivation de clé produit 32 bytes
- `test_encryption_decryption` : Vérifie le chiffrement/déchiffrement
- `test_encrypt_decrypt_vault` : Test end-to-end
- `test_wrong_password` : Vérifie le rejet avec mauvais password
