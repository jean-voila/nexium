use core::fmt;
// use nexium::login::Login;
// use std::io::Write;
use std::fs::{self, File};
use std::io::{BufRead, Read, Write};
// use std::io::Write;
use json;
use std::path::Path;

const DEFAULT_PORT: u16 = 4242;
const DEFAULT_DB_FILE: &str = "./blockchain.db";
const DEFAULT_KEYS_DIR: &str = "./keys";

pub struct Config {
    database_filepath: String,
    keys_filepath: String,

    port: u16,
    // user_id: Login<'a>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            database_filepath: String::from(DEFAULT_DB_FILE),
            keys_filepath: String::from(DEFAULT_KEYS_DIR),
            port: DEFAULT_PORT,
        }
    }

    pub fn from_file(path: &Path) -> Config {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => panic!("this file doesn't exist"),
        };

        let parsed = json::parse(content.as_str()).unwrap();
        Config {
            database_filepath: parsed["database"].to_string(),
            keys_filepath: parsed["keys"].to_string(),
            port: parsed["port"].as_u16().expect("Port is not a number"),
        }
    }

    pub fn to_file(&self, path: &Path) {
        let mut config_obj = json::JsonValue::new_object();
        config_obj["database"] = self.database_filepath.to_string().into();
        config_obj["keys"] = self.keys_filepath.to_string().into();
        config_obj["port"] = self.port.into();
        fs::write(path, config_obj.pretty(4).as_bytes()).expect("Err wwrite");
    }
}

impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\ndatabase: {}\nkeys: {}\nport: {}\n}}",
            self.database_filepath, self.keys_filepath, self.port
        )
    }
}
