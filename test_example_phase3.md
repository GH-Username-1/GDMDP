# Tests de la Phase 3 - Améliorations CLI

## Prérequis

```bash
# Compiler le projet
cargo build --release

# Nettoyer les anciens fichiers de test
rm -f test_vault.dat test_vault*.bak
```

## Test 1 : Générateur de mots de passe

### 1.1 Génération basique

```bash
vault gen
```

**Attendu** :
- Un mot de passe de 16 caractères
- Contient maj, min, chiffres et symboles
- N'a pas de caractères ambigus (0, O, 1, l, I)

### 1.2 Longueur personnalisée

```bash
vault gen --length 32
vault gen -l 8
```

**Attendu** :
- Premier: 32 caractères
- Second: 8 caractères

### 1.3 Options de caractères

```bash
# Seulement chiffres (PIN)
vault gen --no-uppercase --no-lowercase --no-symbols -l 6

# Sans symboles
vault gen --no-symbols

# Avec caractères ambigus
vault gen --include-ambiguous
```

**Attendu** :
- Premier: seulement 6 chiffres
- Second: pas de symboles (!@#$ etc.)
- Troisième: peut contenir 0, O, 1, l, I

### 1.4 Génération multiple

```bash
vault gen --count 10
vault gen -l 20 -c 5
```

**Attendu** :
- Premier: 10 mots de passe numérotés
- Second: 5 mots de passe de 20 caractères

### 1.5 Validation d'erreurs

```bash
# Trop court
vault gen -l 2

# Pas de caractères
vault gen --no-uppercase --no-lowercase --no-digits --no-symbols
```

**Attendu** :
- Erreur: longueur minimale 4 caractères
- Erreur: au moins un type de caractère requis

## Test 2 : Ajout avec génération automatique

```bash
# Initialiser un coffre de test
vault --file test_vault.dat init
# Master password: test123

# Ajouter avec génération
vault --file test_vault.dat add -s GitHub -u user@example.com --generate

# Ajouter avec longueur personnalisée
vault --file test_vault.dat add -s Gmail -u user@gmail.com -g --gen-length 24
```

**Attendu** :
- Affiche "🔑 Mot de passe généré: ..."
- Message "✅ Entrée ajoutée"
- Fichiers de backup créés

## Test 3 : Recherche

```bash
# Ajouter des entrées pour la recherche
vault --file test_vault.dat add -s Slack -u john@work.com -t work,communication
vault --file test_vault.dat add -s Twitter -u @johndoe -t social,personal
vault --file test_vault.dat add -s "GitHub Enterprise" -u admin@company.com -t work,dev

# Rechercher par service
vault --file test_vault.dat search github

# Rechercher par username
vault --file test_vault.dat search john

# Rechercher par tag
vault --file test_vault.dat search work

# Recherche sans résultat
vault --file test_vault.dat search nonexistent
```

**Attendu** :
- "github" trouve GitHub et GitHub Enterprise
- "john" trouve Slack et Twitter
- "work" trouve Slack et GitHub Enterprise
- "nonexistent" affiche "🔍 Aucun résultat"

## Test 4 : Liste avec tags

```bash
# Liste basique
vault --file test_vault.dat list

# Liste avec tags
vault --file test_vault.dat list --tags
```

**Attendu** :
- Première: colonnes ID, Service, Username
- Seconde: colonnes ID, Service, Username, Tags
- Affiche "📊 Total: X entrée(s)"

## Test 5 : Mise à jour d'entrées

### 5.1 Mise à jour de champs individuels

```bash
# Changer le username
vault --file test_vault.dat update GitHub --username new@email.com

# Changer le mot de passe
vault --file test_vault.dat update GitHub --password "manuel123"

# Ajouter une URL
vault --file test_vault.dat update GitHub --url https://github.com/enterprise
```

**Attendu** :
- Chaque commande affiche "✅ Entrée mise à jour: GitHub"
- `vault show GitHub` affiche les nouvelles valeurs

### 5.2 Génération de nouveau mot de passe

```bash
vault --file test_vault.dat update Gmail --generate
```

**Attendu** :
- Affiche "🔑 Nouveau mot de passe généré: ..."
- Le mot de passe est différent de l'original

### 5.3 Mise à jour multiple

```bash
vault --file test_vault.dat update Twitter \
  --username @newhandle \
  --url https://twitter.com/@newhandle \
  --notes "Updated account" \
  --tags social,verified
```

**Attendu** :
- Tous les champs sont mis à jour
- `vault show Twitter` affiche toutes les nouvelles valeurs

### 5.4 Entrée non trouvée

```bash
vault --file test_vault.dat update NonExistant --username test
```

**Attendu** :
- "❌ Aucune entrée trouvée pour: NonExistant"

## Test 6 : Suppression d'entrées

### 6.1 Suppression avec confirmation

```bash
vault --file test_vault.dat delete Twitter
# Répondre: n
```

**Attendu** :
- Demande "⚠️  Êtes-vous sûr de vouloir supprimer 'Twitter'? (y/N)"
- Avec 'n': affiche "Annulé."
- L'entrée existe toujours

```bash
vault --file test_vault.dat delete Twitter
# Répondre: y
```

**Attendu** :
- Avec 'y': "✅ Entrée supprimée: Twitter"
- L'entrée n'existe plus

### 6.2 Suppression sans confirmation

```bash
vault --file test_vault.dat delete Slack --yes
```

**Attendu** :
- Pas de demande de confirmation
- "✅ Entrée supprimée: Slack"

### 6.3 Suppression par ID

```bash
# Obtenir l'ID
vault --file test_vault.dat list
# Copier un ID

vault --file test_vault.dat delete <ID> -y
```

**Attendu** :
- Suppression réussie avec l'ID

## Test 7 : Système de backup

### 7.1 Backups automatiques

```bash
# Lister les backups actuels
ls -la test_vault*.bak

# Faire plusieurs modifications
vault --file test_vault.dat add -s Service1 -u user1 -g
vault --file test_vault.dat add -s Service2 -u user2 -g
vault --file test_vault.dat update Service1 --notes "test"
vault --file test_vault.dat delete Service2 -y

# Relister
ls -la test_vault*.bak
```

**Attendu** :
- Plusieurs fichiers `.bak` avec timestamps
- Format: `test_vault.YYYYMMDD_HHMMSS.bak`

### 7.2 Rotation des backups

```bash
# Créer 10 backups
for i in {1..10}; do
  vault --file test_vault.dat add -s "Service$i" -u "user$i" -g
  sleep 1
done

# Vérifier qu'il n'y a que 5 backups max
ls -la test_vault*.bak | wc -l
```

**Attendu** :
- Maximum 5 fichiers de backup
- Les plus anciens sont supprimés

### 7.3 Backup manuel

```bash
vault --file test_vault.dat backup

# Backup avec conservation de 10
vault --file test_vault.dat backup --keep 10
```

**Attendu** :
- "✅ Backup créé (conservation de X backup(s))"
- Nouveaux fichiers de backup créés

## Test 8 : Affichage amélioré

```bash
vault --file test_vault.dat show GitHub
```

**Attendu** :
- Émojis dans l'affichage:
  - 🔐 Service
  - 👤 Username
  - 🔑 Password
  - 🌐 URL
  - 📝 Notes
  - 🏷️ Tags
  - 🆔 ID
  - 📅 Créé / Modifié
- Dates au format "YYYY-MM-DD HH:MM:SS"

## Test 9 : Validation du master password

```bash
# Tester avec un mot de passe court
vault --file test_short.dat init
# Entrer: "abc"
```

**Attendu** :
- "⚠️  Attention: votre master password est court. Recommandé: 12+ caractères."
- Le coffre est quand même créé

## Test 10 : Compatibilité Phase 2

```bash
# Si vous avez un coffre de la Phase 2
vault list
vault search <terme>
vault update <service> --notes "Test Phase 3"
```

**Attendu** :
- Toutes les commandes fonctionnent
- Le coffre Phase 2 est compatible

## Test 11 : Cas limites

### 11.1 Services avec espaces

```bash
vault --file test_vault.dat add -s "My Bank Account" -u user@bank.com -g
vault --file test_vault.dat show "My Bank Account"
```

**Attendu** :
- Fonctionne correctement avec les espaces

### 11.2 Caractères spéciaux dans les champs

```bash
vault --file test_vault.dat add \
  -s "Test@Service" \
  -u "user+tag@email.com" \
  --notes "Notes with 'quotes' and \"double quotes\"" \
  -g
```

**Attendu** :
- Tous les caractères spéciaux sont préservés

### 11.3 Coffre vide

```bash
vault --file empty.dat init
vault --file empty.dat list
vault --file empty.dat search anything
```

**Attendu** :
- list: "📭 Le coffre-fort est vide."
- search: "🔍 Aucun résultat"

## Test 12 : Performance

```bash
# Ajouter 100 entrées
for i in {1..100}; do
  vault --file big_vault.dat add -s "Service$i" -u "user$i@example.com" -g --gen-length 16
done

# Tester la recherche
time vault --file big_vault.dat search Service50

# Tester le listing
time vault --file big_vault.dat list
```

**Attendu** :
- Les opérations restent rapides même avec beaucoup d'entrées
- La recherche trouve rapidement

## Nettoyage

```bash
# Supprimer tous les fichiers de test
rm -f test_vault.dat test_vault*.bak
rm -f test_short.dat
rm -f empty.dat
rm -f big_vault.dat big_vault*.bak
```

## Résumé des tests

| Test | Fonctionnalité | Statut |
|------|---------------|--------|
| 1 | Générateur de mots de passe | ⬜ |
| 2 | Ajout avec génération | ⬜ |
| 3 | Recherche | ⬜ |
| 4 | Liste avec tags | ⬜ |
| 5 | Mise à jour | ⬜ |
| 6 | Suppression | ⬜ |
| 7 | Backups | ⬜ |
| 8 | Affichage amélioré | ⬜ |
| 9 | Validation password | ⬜ |
| 10 | Compatibilité Phase 2 | ⬜ |
| 11 | Cas limites | ⬜ |
| 12 | Performance | ⬜ |

Cochez les cases (remplacez ⬜ par ✅) au fur et à mesure des tests.
