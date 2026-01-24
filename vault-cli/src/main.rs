use clap::{Parser, Subcommand};
use std::path::PathBuf;
use vault_core::{
    create_backup_with_rotation, decrypt_vault_from_file, encrypt_vault_to_file,
    generate_password, PasswordConfig, Vault, VaultEntry,
};

/// Gestionnaire de mots de passe local sécurisé
#[derive(Parser)]
#[command(name = "vault")]
#[command(about = "Gestionnaire de mots de passe local sécurisé", long_about = None)]
#[command(version)]
struct Cli {
    /// Chemin vers le fichier de coffre-fort
    #[arg(short, long, default_value = "vault.dat", global = true)]
    file: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialise un nouveau coffre-fort
    Init,

    /// Ajoute une nouvelle entrée au coffre-fort
    Add {
        /// Nom du service (ex: GitHub, Gmail)
        #[arg(short, long)]
        service: String,

        /// Nom d'utilisateur ou email
        #[arg(short, long)]
        username: String,

        /// Mot de passe (si non fourni, sera demandé en mode masqué)
        #[arg(short, long)]
        password: Option<String>,

        /// URL du service
        #[arg(short = 'u', long)]
        url: Option<String>,

        /// Notes additionnelles
        #[arg(short, long)]
        notes: Option<String>,

        /// Tags (séparés par des virgules)
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,

        /// Générer automatiquement un mot de passe
        #[arg(short = 'g', long)]
        generate: bool,

        /// Longueur du mot de passe généré (défaut: 16)
        #[arg(long, default_value = "16")]
        gen_length: usize,
    },

    /// Met à jour une entrée existante
    Update {
        /// ID ou nom du service à mettre à jour
        query: String,

        /// Nouveau nom d'utilisateur
        #[arg(short, long)]
        username: Option<String>,

        /// Nouveau mot de passe
        #[arg(short, long)]
        password: Option<String>,

        /// Nouvelle URL
        #[arg(short = 'u', long)]
        url: Option<String>,

        /// Nouvelles notes
        #[arg(short, long)]
        notes: Option<String>,

        /// Nouveaux tags (écrase les anciens)
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,

        /// Générer un nouveau mot de passe
        #[arg(short = 'g', long)]
        generate: bool,
    },

    /// Supprime une entrée
    Delete {
        /// ID ou nom du service à supprimer
        query: String,

        /// Supprimer sans demander de confirmation
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Liste toutes les entrées du coffre-fort
    List {
        /// Afficher les tags
        #[arg(short, long)]
        tags: bool,
    },

    /// Recherche des entrées par service, username ou tag
    Search {
        /// Terme de recherche
        query: String,
    },

    /// Affiche les détails d'une entrée
    Show {
        /// ID ou nom du service
        query: String,
    },

    /// Génère un mot de passe aléatoire
    Gen {
        /// Longueur du mot de passe (défaut: 16)
        #[arg(short, long, default_value = "16")]
        length: usize,

        /// Ne pas utiliser de lettres majuscules
        #[arg(long)]
        no_uppercase: bool,

        /// Ne pas utiliser de lettres minuscules
        #[arg(long)]
        no_lowercase: bool,

        /// Ne pas utiliser de chiffres
        #[arg(long)]
        no_digits: bool,

        /// Ne pas utiliser de symboles
        #[arg(long)]
        no_symbols: bool,

        /// Inclure les caractères ambigus (0, O, 1, l, I)
        #[arg(long)]
        include_ambiguous: bool,

        /// Nombre de mots de passe à générer
        #[arg(short, long, default_value = "1")]
        count: usize,
    },

    /// Crée un backup du coffre-fort
    Backup {
        /// Nombre de backups à conserver (défaut: 5)
        #[arg(short, long, default_value = "5")]
        keep: usize,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("❌ Erreur: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_vault(&cli.file)?,
        Commands::Add {
            service,
            username,
            password,
            url,
            notes,
            tags,
            generate,
            gen_length,
        } => add_entry(
            &cli.file,
            service,
            username,
            password,
            url,
            notes,
            tags,
            generate,
            gen_length,
        )?,
        Commands::Update {
            query,
            username,
            password,
            url,
            notes,
            tags,
            generate,
        } => update_entry(&cli.file, query, username, password, url, notes, tags, generate)?,
        Commands::Delete { query, yes } => delete_entry(&cli.file, query, yes)?,
        Commands::List { tags } => list_entries(&cli.file, tags)?,
        Commands::Search { query } => search_entries(&cli.file, query)?,
        Commands::Show { query } => show_entry(&cli.file, query)?,
        Commands::Gen {
            length,
            no_uppercase,
            no_lowercase,
            no_digits,
            no_symbols,
            include_ambiguous,
            count,
        } => generate_passwords(
            length,
            no_uppercase,
            no_lowercase,
            no_digits,
            no_symbols,
            include_ambiguous,
            count,
        )?,
        Commands::Backup { keep } => backup_vault(&cli.file, keep)?,
    }

    Ok(())
}

fn init_vault(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if path.exists() {
        eprintln!("⚠️  Un coffre-fort existe déjà à cet emplacement.");
        eprintln!("   Utilisez un autre fichier ou supprimez l'existant.");
        return Ok(());
    }

    println!("🔐 Création d'un nouveau coffre-fort...");
    let master_password = read_master_password("Entrez votre master password: ")?;
    let confirm_password = read_master_password("Confirmez votre master password: ")?;

    if master_password != confirm_password {
        return Err("Les mots de passe ne correspondent pas".into());
    }

    if master_password.len() < 8 {
        eprintln!("⚠️  Attention: votre master password est court. Recommandé: 12+ caractères.");
    }

    let vault = Vault::new();
    encrypt_vault_to_file(&vault, path, &master_password)?;

    println!("✅ Coffre-fort créé avec succès: {}", path.display());
    println!("⚠️  IMPORTANT: Conservez votre master password en lieu sûr.");
    println!("   Il n'existe aucun moyen de le récupérer si vous le perdez!");

    Ok(())
}

fn add_entry(
    path: &PathBuf,
    service: String,
    username: String,
    password_opt: Option<String>,
    url: Option<String>,
    notes: Option<String>,
    tags: Vec<String>,
    generate: bool,
    gen_length: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err("Le coffre-fort n'existe pas. Utilisez 'vault init' d'abord.".into());
    }

    let master_password = read_master_password("Master password: ")?;
    let mut vault = decrypt_vault_from_file(path, &master_password)?;

    // Déterminer le mot de passe à utiliser
    let password = if generate {
        let config = PasswordConfig::new(gen_length);
        let generated = generate_password(&config)?;
        println!("🔑 Mot de passe généré: {}", generated);
        generated
    } else {
        match password_opt {
            Some(p) => p,
            None => read_master_password("Mot de passe pour ce service: ")?,
        }
    };

    let entry = VaultEntry::new(service.clone(), username, password, url, notes, tags);
    vault.add_entry(entry);

    // Créer un backup avant la sauvegarde
    create_backup_with_rotation(path, 5).ok();

    encrypt_vault_to_file(&vault, path, &master_password)?;

    println!("✅ Entrée ajoutée: {}", service);

    Ok(())
}

fn update_entry(
    path: &PathBuf,
    query: String,
    username: Option<String>,
    password: Option<String>,
    url: Option<String>,
    notes: Option<String>,
    tags: Option<Vec<String>>,
    generate: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err("Le coffre-fort n'existe pas. Utilisez 'vault init' d'abord.".into());
    }

    let master_password = read_master_password("Master password: ")?;
    let mut vault = decrypt_vault_from_file(path, &master_password)?;

    // Chercher l'entrée
    let entry_id = if let Ok(uuid) = query.parse::<uuid::Uuid>() {
        Some(uuid)
    } else {
        vault
            .list_entries()
            .iter()
            .find(|e| e.service_name.to_lowercase() == query.to_lowercase())
            .map(|e| e.id)
    };

    match entry_id {
        Some(id) => {
            if let Some(entry) = vault.get_entry_mut(&id) {
                let service_name = entry.service_name.clone();

                if let Some(u) = username {
                    entry.username = u;
                }

                if generate {
                    let config = PasswordConfig::default();
                    let generated = generate_password(&config)?;
                    println!("🔑 Nouveau mot de passe généré: {}", generated);
                    entry.password = generated;
                } else if let Some(p) = password {
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

                // Créer un backup avant la sauvegarde
                create_backup_with_rotation(path, 5).ok();

                encrypt_vault_to_file(&vault, path, &master_password)?;

                println!("✅ Entrée mise à jour: {}", service_name);
            }
        }
        None => {
            println!("❌ Aucune entrée trouvée pour: {}", query);
        }
    }

    Ok(())
}

fn delete_entry(
    path: &PathBuf,
    query: String,
    yes: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err("Le coffre-fort n'existe pas. Utilisez 'vault init' d'abord.".into());
    }

    let master_password = read_master_password("Master password: ")?;
    let mut vault = decrypt_vault_from_file(path, &master_password)?;

    // Chercher l'entrée
    let entry_id = if let Ok(uuid) = query.parse::<uuid::Uuid>() {
        Some(uuid)
    } else {
        vault
            .list_entries()
            .iter()
            .find(|e| e.service_name.to_lowercase() == query.to_lowercase())
            .map(|e| e.id)
    };

    match entry_id {
        Some(id) => {
            if let Some(entry) = vault.get_entry(&id) {
                let service_name = entry.service_name.clone();

                if !yes {
                    println!("⚠️  Êtes-vous sûr de vouloir supprimer '{}'? (y/N)", service_name);
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    if !input.trim().eq_ignore_ascii_case("y") {
                        println!("Annulé.");
                        return Ok(());
                    }
                }

                vault.remove_entry(&id);

                // Créer un backup avant la sauvegarde
                create_backup_with_rotation(path, 5).ok();

                encrypt_vault_to_file(&vault, path, &master_password)?;

                println!("✅ Entrée supprimée: {}", service_name);
            }
        }
        None => {
            println!("❌ Aucune entrée trouvée pour: {}", query);
        }
    }

    Ok(())
}

fn list_entries(path: &PathBuf, show_tags: bool) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err("Le coffre-fort n'existe pas. Utilisez 'vault init' d'abord.".into());
    }

    let master_password = read_master_password("Master password: ")?;
    let vault = decrypt_vault_from_file(path, &master_password)?;

    let entries = vault.list_entries();

    if entries.is_empty() {
        println!("📭 Le coffre-fort est vide.");
        return Ok(());
    }

    if show_tags {
        println!(
            "\n{:<36}  {:<25}  {:<25}  {:<20}",
            "ID", "Service", "Username", "Tags"
        );
        println!("{}", "-".repeat(110));

        for entry in entries {
            let tags_str = if entry.tags.is_empty() {
                "-".to_string()
            } else {
                entry.tags.join(", ")
            };
            println!(
                "{:<36}  {:<25}  {:<25}  {:<20}",
                entry.id, entry.service_name, entry.username, tags_str
            );
        }
    } else {
        println!("\n{:<36}  {:<30}  {:<30}", "ID", "Service", "Username");
        println!("{}", "-".repeat(100));

        for entry in entries {
            println!(
                "{:<36}  {:<30}  {:<30}",
                entry.id, entry.service_name, entry.username
            );
        }
    }

    println!("\n📊 Total: {} entrée(s)", entries.len());

    Ok(())
}

fn search_entries(path: &PathBuf, query: String) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err("Le coffre-fort n'existe pas. Utilisez 'vault init' d'abord.".into());
    }

    let master_password = read_master_password("Master password: ")?;
    let vault = decrypt_vault_from_file(path, &master_password)?;

    let results = vault.search(&query);

    if results.is_empty() {
        println!("🔍 Aucun résultat pour: {}", query);
        return Ok(());
    }

    println!("\n🔍 Résultats pour '{}' ({} trouvé(s)):\n", query, results.len());
    println!("{:<36}  {:<30}  {:<30}", "ID", "Service", "Username");
    println!("{}", "-".repeat(100));

    for entry in results {
        println!(
            "{:<36}  {:<30}  {:<30}",
            entry.id, entry.service_name, entry.username
        );
    }

    Ok(())
}

fn show_entry(path: &PathBuf, query: String) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err("Le coffre-fort n'existe pas. Utilisez 'vault init' d'abord.".into());
    }

    let master_password = read_master_password("Master password: ")?;
    let vault = decrypt_vault_from_file(path, &master_password)?;

    // Chercher par ID ou par nom de service
    let entry = if let Ok(uuid) = query.parse::<uuid::Uuid>() {
        vault.get_entry(&uuid)
    } else {
        vault
            .list_entries()
            .iter()
            .find(|e| e.service_name.to_lowercase() == query.to_lowercase())
    };

    match entry {
        Some(e) => {
            println!("\n{}", "=".repeat(60));
            println!("🔐 Service:  {}", e.service_name);
            println!("👤 Username: {}", e.username);
            println!("🔑 Password: {}", e.password);
            if let Some(url) = &e.url {
                println!("🌐 URL:      {}", url);
            }
            if let Some(notes) = &e.notes {
                println!("📝 Notes:    {}", notes);
            }
            if !e.tags.is_empty() {
                println!("🏷️  Tags:     {}", e.tags.join(", "));
            }
            println!("🆔 ID:       {}", e.id);

            let created = chrono::DateTime::from_timestamp(e.created_at, 0);
            let updated = chrono::DateTime::from_timestamp(e.updated_at, 0);
            if let Some(c) = created {
                println!("📅 Créé:     {}", c.format("%Y-%m-%d %H:%M:%S"));
            }
            if let Some(u) = updated {
                println!("📅 Modifié:  {}", u.format("%Y-%m-%d %H:%M:%S"));
            }

            println!("{}", "=".repeat(60));
        }
        None => {
            println!("❌ Aucune entrée trouvée pour: {}", query);
        }
    }

    Ok(())
}

fn generate_passwords(
    length: usize,
    no_uppercase: bool,
    no_lowercase: bool,
    no_digits: bool,
    no_symbols: bool,
    include_ambiguous: bool,
    count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = PasswordConfig {
        length,
        use_uppercase: !no_uppercase,
        use_lowercase: !no_lowercase,
        use_digits: !no_digits,
        use_symbols: !no_symbols,
        exclude_ambiguous: !include_ambiguous,
    };

    println!("\n🔑 Mots de passe générés:\n");

    for i in 0..count {
        let password = generate_password(&config)?;
        if count == 1 {
            println!("{}", password);
        } else {
            println!("{}. {}", i + 1, password);
        }
    }

    println!();

    Ok(())
}

fn backup_vault(path: &PathBuf, keep: usize) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err("Le coffre-fort n'existe pas.".into());
    }

    create_backup_with_rotation(path, keep)?;

    println!("✅ Backup créé (conservation de {} backup(s))", keep);

    Ok(())
}

fn read_master_password(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let password = rpassword::prompt_password(prompt)?;
    Ok(password)
}
