use json;
use nexium::gitlab::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub url_server: String,
    pub port: u16,
    pub user_login: String,
    pub pub_key: String,
    pub priv_key: String,
    pub gitlab_token: String,
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    InvalidFields,
    InvalidLogin,
    InvalidURL,
    InvalidPort,
    InvalidGitlabToken,
    NetworkError,
}

const CONFIG_FILE_PATH: &str = "config_client.json";

impl Config {
    /// Create a new Config from a json file
    pub fn from_file(path: &Path) -> Result<Config, ConfigError> {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                return Err(ConfigError::FileNotFound);
            }
        };

        let parsed = json::parse(content.as_str()).unwrap();
        if parsed["url_server"].is_null()
            || parsed["port"].is_null()
            || parsed["user_login"].is_null()
            || parsed["pub_key"].is_null()
            || parsed["priv_key"].is_null()
            || parsed["gitlab_token"].is_null()
        {
            return Err(ConfigError::InvalidFields);
        }

        Ok(Config {
            url_server: parsed["url_server"].to_string(),
            port: parsed["port"]
                .as_u16()
                .expect("Config read: Port is not a number"),
            user_login: parsed["user_login"].to_string(),
            pub_key: parsed["pub_key"].to_string(),
            priv_key: parsed["priv_key"].to_string(),
            gitlab_token: parsed["gitlab_token"].to_string(),
        })
    }

    /// Write the Config object to a json filepriv_keyb fn to_filriv&self, path: &Path)
    pub fn to_file(&self, path: &Path) {
        let mut config_obj = json::JsonValue::new_object();
        config_obj["url_server"] = self.url_server.to_string().into();
        config_obj["port"] = self.port.into();
        config_obj["user_login"] = self.user_login.to_string().into();
        config_obj["pub_key"] = self.pub_key.to_string().into();
        config_obj["priv_key"] = self.priv_key.to_string().into();
        config_obj["gitlab_token"] = self.gitlab_token.to_string().into();
        fs::write(path, config_obj.pretty(4).as_bytes())
            .expect("Error writing config file");
    }

    pub fn check_values(
        port: u16,
        url: String,
        login: String,
        gitlab_token: String,
    ) -> Result<(), ConfigError> {
        // Check if port is valid port

        if port < 1 {
            return Err(ConfigError::InvalidPort);
        }

        // Check if url_server is valid url
        if url.contains(" ")
            || url.contains("\n")
            || url.contains("..")
            || url.ends_with(".")
            || url.starts_with(".")
        {
            return Err(ConfigError::InvalidURL);
        }

        // Check if gitlab_token is valid token
        let gitlab_client = GitlabClient::new(gitlab_token.clone());
        match gitlab_client.check_token() {
            Ok(valid) => {
                if !valid {
                    return Err(ConfigError::InvalidGitlabToken);
                }
            }
            Err(_) => {
                return Err(ConfigError::NetworkError);
            }
        };

        // Check if  user_login is valid login (prenom.nom)
        let parts: Vec<&str> = login.split('.').collect();
        if parts.len() != 2 {
            return Err(ConfigError::InvalidLogin);
        }
        if parts[0].len() == 0 || parts[1].len() == 0 {
            return Err(ConfigError::InvalidLogin);
        }

        return Ok(());
    }
}
