use json;
use std::fs;
use std::io;
use std::io::Write;
// use std::io::prelude::*;

use nexium::gitlab;
use nexium::login;
use std::path::Path;

const DEFAULT_PORT: u16 = 4242;
const DEFAULT_DB_FILE: &str = "blockchain.db";
const DEFAULT_KEYS_DIR: &str = "keys/";

/// Config struct to hold the configuration of the server

#[derive(Debug)]
pub struct Config {
    /// Path to the database file
    database_filepath: String,
    /// Path to the directory containing the keys
    keys_filepath: String,
    /// Port on which the server will listen
    port: u16,
    /// User login to use for the server
    user_login: login::Login,
    /// Gitlab Token for the user
    gitlab_token: gitlab::GitlabToken,
}

impl Config {
    /// Create a new Config object with default values
    pub fn generate() -> Config {
        let ulogin: login::Login =
            match login::Login::from(ask("Enter user id: ")) {
                Some(l) => l,
                None => {
                    panic!("Invalid user id");
                }
            };

        let token = ask("Enter Gitlab token: ");
        if token.is_empty() {
            panic!("Empty Gitlab token");
        }

        let port: u16 =
            match ask(&format!("Enter port (default: {}): ", DEFAULT_PORT))
                .parse()
            {
                Ok(p) => p,
                Err(_) => {
                    println!("Empty or invalid port, using default");
                    DEFAULT_PORT
                }
            };

        let database_filepath = match ask(&format!(
            "Enter database file path (default: {}): ",
            DEFAULT_DB_FILE
        ))
        .as_str()
        {
            "" => {
                println!("Empty path, using default");
                String::from(DEFAULT_DB_FILE)
            }
            s => s.to_string(),
        };

        let keys_filepath = match ask(&format!(
            "Enter keys directory path (default: {}): ",
            DEFAULT_KEYS_DIR
        ))
        .as_str()
        {
            "" => {
                println!("Empty path, using default");
                String::from(DEFAULT_KEYS_DIR)
            }
            s => s.to_string(),
        };

        let gitlab_token = gitlab::GitlabToken::new(token, ulogin.clone());
        if !gitlab_token.check_token() {
            panic!("Invalid token");
        }

        return Config {
            database_filepath,
            keys_filepath,
            port,
            user_login: ulogin,
            gitlab_token,
        };
    }

    /// Create a new Config from a json file
    pub fn from_file(path: &Path) -> Config {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => panic!("Error reading config file"),
        };

        let parsed = json::parse(content.as_str()).unwrap();
        if parsed["database"].is_null()
            || parsed["keys"].is_null()
            || parsed["port"].is_null()
            || parsed["user_id"].is_null()
            || parsed["gitlab_token"].is_null()
            || !parsed["port"].is_number()
        {
            panic!("Error parsing config file");
        }

        let token = gitlab::GitlabToken::new(
            parsed["gitlab_token"].to_string(),
            login::Login::from(parsed["user_id"].to_string())
                .expect("Config read: Invalid user id"),
        );

        if !token.check_token() {
            panic!("Invalid token");
        }

        Config {
            database_filepath: parsed["database"].to_string(),
            keys_filepath: parsed["keys"].to_string(),
            port: parsed["port"]
                .as_u16()
                .expect("Config read: Port is not a number"),
            user_login: login::Login::from(parsed["user_id"].to_string())
                .expect("Config read: Invalid user id"),
            gitlab_token: token,
        }
    }

    /// Write the Config object to a json file
    pub fn to_file(&self, path: &Path) {
        let mut config_obj = json::JsonValue::new_object();
        config_obj["database"] = self.database_filepath.to_string().into();
        config_obj["keys"] = self.keys_filepath.to_string().into();
        config_obj["port"] = self.port.into();
        config_obj["user_id"] = self.user_login.to_string().into();
        config_obj["gitlab_token"] = self.gitlab_token.get_token().into();
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
