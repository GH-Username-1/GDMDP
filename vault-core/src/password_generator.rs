use rand::Rng;

/// Configuration pour la génération de mots de passe
#[derive(Debug, Clone)]
pub struct PasswordConfig {
    pub length: usize,
    pub use_uppercase: bool,
    pub use_lowercase: bool,
    pub use_digits: bool,
    pub use_symbols: bool,
    pub exclude_ambiguous: bool, // Exclure 0, O, l, 1, I, etc.
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            length: 16,
            use_uppercase: true,
            use_lowercase: true,
            use_digits: true,
            use_symbols: true,
            exclude_ambiguous: true,
        }
    }
}

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";

const AMBIGUOUS_CHARS: &str = "0O1lI";

impl PasswordConfig {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            ..Default::default()
        }
    }

    /// Construit le jeu de caractères en fonction de la configuration
    fn build_charset(&self) -> String {
        let mut charset = String::new();

        if self.use_lowercase {
            charset.push_str(LOWERCASE);
        }
        if self.use_uppercase {
            charset.push_str(UPPERCASE);
        }
        if self.use_digits {
            charset.push_str(DIGITS);
        }
        if self.use_symbols {
            charset.push_str(SYMBOLS);
        }

        // Exclure les caractères ambigus si demandé
        if self.exclude_ambiguous {
            charset = charset
                .chars()
                .filter(|c| !AMBIGUOUS_CHARS.contains(*c))
                .collect();
        }

        charset
    }

    /// Valide la configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.length < 4 {
            return Err("La longueur minimale est de 4 caractères".to_string());
        }
        if self.length > 128 {
            return Err("La longueur maximale est de 128 caractères".to_string());
        }
        if !self.use_lowercase
            && !self.use_uppercase
            && !self.use_digits
            && !self.use_symbols
        {
            return Err("Au moins un type de caractère doit être activé".to_string());
        }
        Ok(())
    }
}

/// Génère un mot de passe aléatoire selon la configuration
pub fn generate_password(config: &PasswordConfig) -> Result<String, String> {
    config.validate()?;

    let charset = config.build_charset();
    if charset.is_empty() {
        return Err("Le jeu de caractères est vide".to_string());
    }

    let charset_chars: Vec<char> = charset.chars().collect();
    let mut rng = rand::thread_rng();

    // Générer le mot de passe
    let password: String = (0..config.length)
        .map(|_| {
            let idx = rng.gen_range(0..charset_chars.len());
            charset_chars[idx]
        })
        .collect();

    // Vérifier qu'on a au moins un caractère de chaque type activé
    if config.use_lowercase && !password.chars().any(|c| LOWERCASE.contains(c)) {
        // Re-générer récursivement (rare)
        return generate_password(config);
    }
    if config.use_uppercase && !password.chars().any(|c| UPPERCASE.contains(c)) {
        return generate_password(config);
    }
    if config.use_digits && !password.chars().any(|c| DIGITS.contains(c)) {
        return generate_password(config);
    }
    if config.use_symbols && !password.chars().any(|c| SYMBOLS.contains(c)) {
        return generate_password(config);
    }

    Ok(password)
}

/// Génère un mot de passe avec les paramètres par défaut
pub fn generate_default_password() -> String {
    generate_password(&PasswordConfig::default())
        .expect("Default config should always work")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_password_generation() {
        let password = generate_default_password();
        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_custom_length() {
        let config = PasswordConfig::new(20);
        let password = generate_password(&config).unwrap();
        assert_eq!(password.len(), 20);
    }

    #[test]
    fn test_only_digits() {
        let config = PasswordConfig {
            length: 10,
            use_uppercase: false,
            use_lowercase: false,
            use_digits: true,
            use_symbols: false,
            exclude_ambiguous: false,
        };
        let password = generate_password(&config).unwrap();
        assert!(password.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_exclude_ambiguous() {
        let config = PasswordConfig {
            length: 20,
            use_uppercase: true,
            use_lowercase: true,
            use_digits: true,
            use_symbols: false,
            exclude_ambiguous: true,
        };
        let password = generate_password(&config).unwrap();
        assert!(!password.chars().any(|c| AMBIGUOUS_CHARS.contains(c)));
    }

    #[test]
    fn test_minimum_length_validation() {
        let config = PasswordConfig::new(2);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_no_charset_validation() {
        let config = PasswordConfig {
            length: 10,
            use_uppercase: false,
            use_lowercase: false,
            use_digits: false,
            use_symbols: false,
            exclude_ambiguous: false,
        };
        assert!(config.validate().is_err());
    }
}
