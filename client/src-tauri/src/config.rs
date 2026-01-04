use nexium::gitlab;
use nexium::gitlab::*;

use nexium::login::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

fn get_config_path() -> PathBuf {
    let mut path = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("nexium");
    fs::create_dir_all(&path).ok();
    path.push("config.json");
    path
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server_address: String,
    pub port: String,
    pub user_login: String,
    pub pub_key: String,
    pub priv_key: String,
    pub gitlab_token: String,
    pub gitlab_token_type: gitlab::TokenType,
    pub is_testnet: bool,
    pub password: String,
    pub server_login: String,
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    InvalidAddress,
    InvalidPort,
    InvalidGitlabToken,
    NetworkError,
    InternalError,
    FileFormatError,
    FileWriteError,
    KeyGenerationError,
    EmptyKeyError,
    FileReadError,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ConfigError::FileNotFound => {
                "Fichier de configuration introuvable."
            }

            ConfigError::InvalidAddress => "Adresse de serveur invalide.",
            ConfigError::InvalidPort => "Port de serveur invalide.",
            ConfigError::InvalidGitlabToken => "Token GitLab invalide.",
            ConfigError::NetworkError => {
                "Erreur de réseau lors de la vérification du token."
            }
            ConfigError::InternalError => "Erreur interne.",
            ConfigError::FileFormatError => {
                "Le format du fichier de configuration est invalide."
            }
            ConfigError::FileWriteError => {
                "Erreur d'écriture dans le fichier de configuration."
            }
            ConfigError::KeyGenerationError => "Erreur de génération de clé.",
            ConfigError::EmptyKeyError => "Une ou plusieurs clés sont vides.",
            ConfigError::FileReadError => {
                "Erreur de lecture du fichier de configuration."
            }
        };
        write!(f, "{msg}")
    }
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Config, String> {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                return Err(ConfigError::FileNotFound.to_string());
            }
        };
        let config: Config = match serde_json::from_str(&content) {
            Ok(c) => c,
            Err(_) => {
                return Err(ConfigError::FileFormatError.to_string());
            }
        };
        match config.check_values() {
            Ok(_) => {}
            Err(e) => {
                return Err(e.to_string());
            }
        }

        return Ok(config);
    }

    pub fn to_file(&self, path: &Path) -> Result<(), ConfigError> {
        let content = match serde_json::to_string_pretty(self) {
            Ok(c) => c,
            Err(_) => {
                return Err(ConfigError::FileFormatError);
            }
        };
        match fs::write(path, content) {
            Ok(_) => {}
            Err(_) => {
                return Err(ConfigError::FileWriteError);
            }
        }
        return Ok(());
    }

    pub fn check_values(&self) -> Result<(), String> {
        match Login::new(self.user_login.clone()) {
            Ok(_) => {}
            Err(e) => {
                return Err(e.to_string());
            }
        }

        match self.port.parse::<u16>() {
            Ok(_) => {}
            Err(_) => {
                return Err(ConfigError::InvalidPort.to_string());
            }
        }

        if self.server_address.len() == 0 {
            return Err(ConfigError::InvalidAddress.to_string());
        }

        if self.server_address.contains(":") {
            return Err(ConfigError::InvalidAddress.to_string());
        }

        if self.server_address.contains("..") {
            return Err(ConfigError::InvalidAddress.to_string());
        }

        let gitlab_client = GitlabClient::new(
            self.gitlab_token.clone(),
            self.gitlab_token_type.clone(),
        );

        match gitlab_client.check_token() {
            Ok(valid) => {
                if !valid {
                    return Err(ConfigError::InvalidGitlabToken.to_string());
                }
            }
            Err(e) => {
                return Err(match e {
                    GitlabError::NetworkError => ConfigError::NetworkError,
                    GitlabError::InvalidToken => {
                        ConfigError::InvalidGitlabToken
                    }
                    _ => ConfigError::InternalError,
                }
                .to_string());
            }
        };

        let login = Login::new(self.user_login.clone());
        match login {
            Ok(_) => {}
            Err(e) => {
                return Err(e.to_string());
            }
        }

        if self.pub_key.len() == 0 || self.priv_key.len() == 0 {
            return Err(ConfigError::EmptyKeyError.to_string());
        }

        return Ok(());
    }

    pub fn load() -> Option<Self> {
        let path = get_config_path();
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return Some(config);
            }
        }
        None
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let path = get_config_path();
        let content = serde_json::to_string_pretty(self)
            .map_err(|_| ConfigError::FileFormatError)?;
        fs::write(&path, content)
            .map_err(|_| ConfigError::FileWriteError)?;
        Ok(())
    }
}
