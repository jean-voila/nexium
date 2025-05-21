use nexium::gitlab;
use nexium::gitlab::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;
use std::fs;
use std::path::Path;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub url_server: String,
    pub port: u16,
    pub user_login: String,
    pub pub_key: String,
    pub priv_key: String,
    pub gitlab_token: String,
    pub gitlab_token_type: gitlab::TokenType,
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    InvalidLogin,
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
            ConfigError::FileNotFound => "Configuration file not found.",

            ConfigError::InvalidLogin => {
                "Invalid login format. Expected 'prenom.nom'."
            }
            ConfigError::InvalidURL => "Invalid server URL.",
            ConfigError::InvalidPort => "Invalid port number.",
            ConfigError::InvalidGitlabToken => "Invalid GitLab token.",
            ConfigError::NetworkError => {
                "Network error while validating GitLab token."
            }
            ConfigError::InternalError => "Internal error.",
            ConfigError::FileFormatError => {
                "Configuration file format is invalid."
            }
            ConfigError::FileWriteError => {
                "Error writing to the configuration file."
            }
            ConfigError::KeyGenerationError => "Error generating key pair.",
            ConfigError::EmptyKeyError => "Public or private key is empty.",
            ConfigError::FileReadError => {
                "Error reading the configuration file."
            }
        };
        write!(f, "{msg}")
    }
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Config, ConfigError> {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                return Err(ConfigError::FileNotFound);
            }
        };
        let config: Config = match serde_json::from_str(&content) {
            Ok(c) => c,
            Err(_) => {
                return Err(ConfigError::FileFormatError);
            }
        };
        match config.check_values() {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
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

    pub fn check_values(&self) -> Result<(), ConfigError> {
        if self.port < 1 {
            return Err(ConfigError::InvalidPort);
        }

        match Url::parse(&self.url_server) {
            Ok(_) => {}
            Err(_) => {
                return Err(ConfigError::InvalidURL);
            }
        }

        let gitlab_client = GitlabClient::new(
            self.gitlab_token.clone(),
            self.gitlab_token_type.clone(),
        );

        match gitlab_client.check_token() {
            Ok(valid) => {
                if !valid {
                    return Err(ConfigError::InvalidGitlabToken);
                }
            }
            Err(e) => {
                return Err(match e {
                    GitlabError::NetworkError => ConfigError::NetworkError,
                    GitlabError::InvalidToken => {
                        ConfigError::InvalidGitlabToken
                    }
                    _ => ConfigError::InternalError,
                });
            }
        };

        let parts: Vec<&str> = self.user_login.split('.').collect();
        if parts.len() != 2 || parts[0].len() == 0 || parts[1].len() == 0 {
            return Err(ConfigError::InvalidLogin);
        }

        if self.pub_key.len() == 0 || self.priv_key.len() == 0 {
            return Err(ConfigError::EmptyKeyError);
        }

        return Ok(());
    }
}
