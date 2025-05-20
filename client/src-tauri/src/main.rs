// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod config;
use config::Config;
use nexium::gitlab::*;

// use sleep
use std::path::Path;

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
        return Err("File does not exist".to_string());
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
        ])
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
