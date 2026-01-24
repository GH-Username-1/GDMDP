# Guide de Démarrage Rapide

## Installation de Rust (si nécessaire)

### Windows
1. Téléchargez l'installateur depuis https://rustup.rs/
2. Exécutez `rustup-init.exe`
3. Suivez les instructions à l'écran
4. Redémarrez votre terminal

### Vérification
```bash
rustc --version
cargo --version
```

## Compilation du projet

```bash
# Dans le dossier GDMDP_
cargo build --release
```

Le binaire sera créé dans : `target/release/vault.exe` (Windows) ou `target/release/vault` (Linux/macOS)

## Premier test

### 1. Initialiser un coffre-fort

```bash
# Windows
.\target\release\vault.exe init

# Linux/macOS
./target/release/vault init
```

Vous serez invité à créer un master password. Choisissez-le fort et mémorisez-le bien !

### 2. Ajouter votre première entrée

```bash
.\target\release\vault.exe add --service GitHub --username votre@email.com
```

Le CLI vous demandera :
- Votre master password
- Le mot de passe pour GitHub (en mode masqué)

### 3. Lister vos entrées

```bash
.\target\release\vault.exe list
```

### 4. Afficher les détails

```bash
.\target\release\vault.exe show GitHub
```

## Utilisation pratique

### Ajouter l'exécutable au PATH (optionnel)

#### Windows
```powershell
# Copiez le binaire dans un dossier de votre PATH
copy target\release\vault.exe C:\Users\VotreNom\bin\

# Ou ajoutez le dossier target\release au PATH
```

#### Linux/macOS
```bash
# Créer un lien symbolique
sudo ln -s $(pwd)/target/release/vault /usr/local/bin/vault

# Ou ajouter au PATH dans ~/.bashrc ou ~/.zshrc
export PATH="$PATH:$(pwd)/target/release"
```

Après cela, vous pourrez utiliser simplement `vault` au lieu de `./target/release/vault`

## Exemples d'utilisation

```bash
# Ajouter une entrée complète
vault add \
  --service Gmail \
  --username john@gmail.com \
  --url https://mail.google.com \
  --notes "Compte personnel" \
  --tags email,personal

# Utiliser un fichier de coffre personnalisé
vault --file travail.dat init
vault --file travail.dat add --service Slack --username john@company.com

# Afficher une entrée spécifique
vault show Gmail
```

## Sécurité - Points importants

1. **Master Password** : C'est LA clé de votre coffre. Pas de récupération possible si vous le perdez !
2. **Backup** : Faites des copies de votre fichier `vault.dat` régulièrement
3. **Ne commitez jamais** les fichiers `.dat` dans git
4. **Permissions** : Sur Linux/macOS, protégez vos fichiers :
   ```bash
   chmod 600 vault.dat
   ```

## Troubleshooting

### "cargo: command not found"
→ Rust n'est pas installé ou pas dans le PATH. Installez Rust via rustup.rs

### "Le coffre-fort n'existe pas"
→ Initialisez d'abord avec `vault init`

### "Invalid master password"
→ Vérifiez votre master password. Pas de récupération possible si vous l'avez oublié.

### Erreur de compilation
→ Vérifiez que vous avez Rust 1.70+ : `rustc --version`
→ Nettoyez et recompilez : `cargo clean && cargo build --release`
