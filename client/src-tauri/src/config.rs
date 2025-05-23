use nexium::gitlab;
use nexium::gitlab::*;
use nexium::login;
use nexium::login::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;
use std::fs;
use std::path::Path;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub url_server: String,
    pub port: Option<u16>,
    pub user_login: String,
    pub pub_key: String,
    pub priv_key: String,
    pub gitlab_token: String,
    pub gitlab_token_type: gitlab::TokenType,
    pub is_testnet: bool,
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,

    InvalidURL,
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

            ConfigError::InvalidURL => "URL de serveur invalide.",
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
        match self.port {
            Some(port) => {
                if port < 1 {
                    return Err(ConfigError::InvalidPort.to_string());
                }
            }
            None => {
                return Err(ConfigError::InvalidPort.to_string());
            }
        }

        match Url::parse(&self.url_server) {
            Ok(_) => {}
            Err(_) => {
                return Err(ConfigError::InvalidURL.to_string());
            }
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
}
