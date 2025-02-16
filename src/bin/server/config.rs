use json;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

const DEFAULT_PATH: &str = "./test_data/config.json";

const DEFAULT_PORT: u16 = 4242;
const DEFAULT_DB_FILE: &str = "./blockchain.db";
const DEFAULT_KEYS_DIR: &str = "./keys";

/// Config struct to hold the configuration of the server
pub struct Config {
    /// Path to the database file
    database_filepath: String,
    /// Path to the directory containing the keys
    keys_filepath: String,
    /// Port on which the server will listen
    port: u16,
    /// User login to use for the server
    user_id: String,
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Error reading line");
    return line.trim().to_string();
}

pub fn generate_config() {
    println!("Where to generate config? (default: {DEFAULT_PATH})");

    let line = read_line();

    let p = if line.len() == 0 {
        Path::new(DEFAULT_PATH)
    } else {
        Path::new(&line)
    };

    if p.exists() {
        println!("File already exists, overwrite? (y/n) ");
        let line = read_line();

        if line.len() == 0 || line.chars().next().unwrap() != 'y' {
            println!("Aborting config generation");
            return;
        }
    }

    println!("Enter user id: ");
    let user_id = read_line();
    if user_id.len() == 0 {
        println!("User id cannot be empty, aborting config generation");
        return;
    }

    // user_id check ?

    let mut config = Config::new();
    config.user_id = user_id;
    config.to_file(p);
    println!("Config generated with default values at '{}'", p.display());
}

impl Config {
    /// Create a new Config object with default values
    pub fn new() -> Config {
        Config {
            database_filepath: String::from(DEFAULT_DB_FILE),
            keys_filepath: String::from(DEFAULT_KEYS_DIR),
            port: DEFAULT_PORT,
            user_id: String::from(""),
        }
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
        {
            panic!("Error parsing config file");
        }

        Config {
            database_filepath: parsed["database"].to_string(),
            keys_filepath: parsed["keys"].to_string(),
            port: parsed["port"].as_u16().expect("Port is not a number"),
            user_id: parsed["user_id"].to_string(),
        }
    }

    /// Write the Config object to a json file
    pub fn to_file(&self, path: &Path) {
        let mut config_obj = json::JsonValue::new_object();
        config_obj["database"] = self.database_filepath.to_string().into();
        config_obj["keys"] = self.keys_filepath.to_string().into();
        config_obj["port"] = self.port.into();
        config_obj["user_id"] = self.user_id.to_string().into();
        fs::write(path, config_obj.pretty(4).as_bytes())
            .expect("Error writing config file");
    }
}

impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\ndatabase: {}\nkeys: {}\nport: {}\nuser_id: {}\n}}",
            self.database_filepath, self.keys_filepath, self.port, self.user_id
        )
    }
}
