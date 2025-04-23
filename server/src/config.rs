use json;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

/// Default port to listen on
const DEFAULT_PORT: u16 = 4242;
/// Default path to database file
const DEFAULT_DB_FILE: &str = "blockchain.db";
/// Default path to key file
const DEFAULT_KEY_PATH: &str = "private-key.pem";
/// Default path to the Gitlab API URL
const DEFAULT_GITLAB_API_URL: &str = "https://gitlab.cri.epita.fr/api/v4";

/// Config struct to hold the configuration of the server

#[derive(Debug)]
pub struct Config {
    /// Path to the database file
    pub database_filepath: String,
    /// Path to the directory containing the key
    pub key_filepath: String,
    /// Port on which the server will listen
    pub port: u16,
    /// User login to use for the server
    pub user_login: String,
    /// Gitlab Token for the user
    pub gitlab_token: String,
    /// Gitlab API URL
    pub gitlab_api_url: String,
}

impl Config {
    /// Create a new Config object with default values
    pub fn generate(path: &Path) -> Config {
        let user_login = ask("Enter your login (format: first.last): ");
        if !_check_login_syntax(user_login.clone()) {
            panic!("Invalid login format");
        }

        let gitlab_token = match env::var("GITLAB_TOKEN") {
            Ok(t) => {
                println!("Using GITLAB_TOKEN from environment variable");
                t
            }
            Err(_) => match ask("Enter Gitlab token: ").as_str() {
                "" => {
                    println!("Empty token, using default");
                    String::from("")
                }
                s => s.to_string(),
            },
        };

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

        let key_filepath = match ask(&format!(
            "Enter key directory path (default: {}): ",
            DEFAULT_KEY_PATH
        ))
        .as_str()
        {
            "" => {
                println!("Empty path, using default");
                String::from(DEFAULT_KEY_PATH)
            }
            s => s.to_string(),
        };

        let gitlab_api_url = match ask(&format!(
            "Enter Gitlab API URL (default: {}): ",
            DEFAULT_GITLAB_API_URL
        ))
        .as_str()
        {
            "" => {
                println!("Empty path, using default");
                String::from(DEFAULT_GITLAB_API_URL)
            }
            s => s.to_string(),
        };

        let res = Config {
            database_filepath,
            key_filepath,
            port,
            user_login,
            gitlab_token,
            gitlab_api_url,
        };

        res.to_file(path);

        return res;
    }

    /// Create a new Config from a json file
    pub fn from_file(path: &Path) -> Config {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => panic!("Error reading config file: file not found. Try --generate-config."),
        };

        let parsed = json::parse(content.as_str()).unwrap();
        if parsed["database"].is_null()
            || parsed["key"].is_null()
            || parsed["port"].is_null()
            || parsed["user_id"].is_null()
            || parsed["gitlab_token"].is_null()
            || parsed["gitlab_api_url"].is_null()
            || !parsed["port"].is_number()
        {
            panic!("Error parsing config file");
        }

        let gitlab_token = match parsed["gitlab_token"].as_str() {
            Some(t) => t.to_string(),
            None => panic!("Error parsing gitlab token"),
        };

        let gitlab_api_url = match parsed["gitlab_api_url"].as_str() {
            Some(t) => t.to_string(),
            None => panic!("Error parsing gitlab api url"),
        };

        Config {
            database_filepath: parsed["database"].to_string(),
            key_filepath: parsed["key"].to_string(),
            port: parsed["port"]
                .as_u16()
                .expect("Config read: Port is not a number"),
            user_login: parsed["user_id"].to_string(),
            gitlab_token,
            gitlab_api_url,
        }
    }

    /// Write the Config object to a json file
    pub fn to_file(&self, path: &Path) {
        let mut config_obj = json::JsonValue::new_object();
        config_obj["database"] = self.database_filepath.to_string().into();
        config_obj["key"] = self.key_filepath.to_string().into();
        config_obj["port"] = self.port.into();
        config_obj["user_id"] = self.user_login.to_string().into();
        config_obj["gitlab_token"] = self.gitlab_token.to_string().into();
        config_obj["gitlab_api_url"] = self.gitlab_api_url.to_string().into();
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
