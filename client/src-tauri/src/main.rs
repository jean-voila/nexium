// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod config;
use config::Config;
use config::ConfigError;
use nexium::gitlab;
use nexium::{gitlab::*, rsa::*};
// use sleep
use std::{f32::consts::E, path::Path};

const DEFAULT_KEY_BITLENGTH: usize = 2048;

#[tauri::command]
fn check_config_values(
    port: String,
    url: String,
    login: String,
    gitlabtoken: String,
    tokentypestring: String,
) -> Result<String, String> {
    // sleep for 2 seconds
    match Config::check_values(port, url, login, gitlabtoken, tokentypestring) {
        Ok(_) => Ok("".to_string()),
        Err(e) => Err(format!("{}", e)),
    }
}
#[tauri::command]
fn load_config_from_file(path_string: String) -> Result<Config, String> {
    // check if file exists
    if Path::new(&path_string).exists() == false {
        //fmt is implemented for ConfigError (enum)
        // so we can use format! macro to create a string
        // with the error message
        return Err(format!("{}", ConfigError::FileNotFound));
    }

    let path = Path::new(&path_string);
    match Config::from_file(path) {
        Ok(config) => Ok(config),
        Err(e) => Err(format!("{}", e)),
    }
}

#[tauri::command]
fn save_config_to_file(
    path_string: String,
    port: u16,
    url: String,
    login: String,
    gitlab_token: String,
    pub_key: String,
    priv_key: String,
) -> Result<String, String> {
    let path = Path::new(&path_string);
    let config = config::Config {
        port: port,
        url_server: url,
        user_login: login,
        gitlab_token: gitlab_token,
        pub_key: pub_key,
        priv_key: priv_key,
    };
    match Config::to_file(&config, path) {
        Ok(_) => Ok("".to_string()),
        Err(e) => Err(format!("{}", e)),
    }
}

#[tauri::command]
fn keypair_generation(
    login: String,
    password: String,
) -> Result<(String, String), String> {
    let keypair = KeyPair::generate(DEFAULT_KEY_BITLENGTH, &login);
    let pub_key = KeyPair::pub_to_pem(&keypair);
    let priv_key = KeyPair::priv_to_pem(&keypair, &password);
    if pub_key == "" || priv_key == "" {
        return Err(format!("{}", ConfigError::KeyGenerationError));
    }
    Ok((pub_key, priv_key))
}

#[tauri::command]
fn send_gpg_key(
    tokentypestring: String,
    gitlab_token: String,
    pub_key: String,
) -> Result<String, String> {
    let tokentype = match tokentypestring.as_str() {
        "classic" => TokenType::Classic,
        "oauth" => TokenType::OAuth,
        _ => {
            return Err(format!("{}", ConfigError::InternalError));
        }
    };
    let gitlab_client = GitlabClient::new(gitlab_token, tokentype);
    match gitlab_client.add_gpg_key(&pub_key) {
        Ok(_) => Ok(("").to_string()),
        Err(e) => Err(format!("{}", e)),
    }
}

#[tauri::command]
fn get_gitlab_oauth_token() -> Result<String, String> {
    match GitlabClient::get_token() {
        Ok(token) => Ok(token),
        Err(e) => Err(format!("{:?}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            check_config_values,
            get_gitlab_oauth_token,
            load_config_from_file,
            save_config_to_file,
            keypair_generation,
            send_gpg_key
        ])
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
