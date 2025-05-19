use json;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;
use std::io::Write;
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

pub enum ConfigError {
    FileNotFound,
    InvalidFields,
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
        config_obj["pub_key"] = self.pub_key.to_string().into();
        config_obj["priv_key"] = self.priv_key.to_string().into();
        config_obj["port"] = self.port.into();
        config_obj["user_first_name"] = self.user_login.to_string().into();
        config_obj["gitlab_token"] = self.gitlab_token.to_string().into();
        fs::write(path, config_obj.pretty(4).as_bytes())
            .expect("Error writing config file");
    }
}

fn ask(ask: &str) -> String {
    print!("{}", ask);
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Error reading line");

    return line.trim().to_string();
}

fn _check_login_syntax(login: String) -> bool {
    let parts: Vec<&str> = login.split('.').collect();
    if parts.len() != 2 {
        return false;
    }
    if parts[0].len() == 0 || parts[1].len() == 0 {
        return false;
    }
    return true;
}
