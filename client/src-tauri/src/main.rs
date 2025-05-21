// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod config;
use config::Config;
use config::ConfigError;

use nexium::{gitlab::*, rsa::*};
// use sleep
use std::path::Path;

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
async fn load_config_from_file(path_string: String) -> Result<Config, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        if Path::new(&path_string).exists() == false {
            return Err(format!("{}", ConfigError::FileNotFound));
        }
        let path = Path::new(&path_string);
        match Config::from_file(path) {
            Ok(config) => Ok(config),
            Err(e) => Err(format!("{}", e)),
        }
    })
    .await;

    match result {
        Ok(r) => r,
        Err(_) => Err("Erreur lors du chargement du fichier".into()),
    }
}

#[tauri::command]
async fn save_config_to_file(
    path_string: String,
    port: u16,
    url: String,
    login: String,
    gitlab_token: String,
    pub_key: String,
    priv_key: String,
) -> Result<String, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        let path = Path::new(&path_string);
        let config = config::Config {
            port,
            url_server: url,
            user_login: login,
            gitlab_token,
            pub_key,
            priv_key,
        };
        match Config::to_file(&config, path) {
            Ok(_) => Ok("".to_string()),
            Err(e) => Err(format!("{}", e)),
        }
    })
    .await;

    match result {
        Ok(r) => r,
        Err(_) => Err("Erreur lors de la sauvegarde du fichier".into()),
    }
}

#[tauri::command]
async fn keypair_generation(
    login: String,
    password: String,
) -> Result<(String, String), String> {
    // Utilise spawn_blocking pour éviter de bloquer le thread principal
    let result = tauri::async_runtime::spawn_blocking(move || {
        let keypair = KeyPair::generate(DEFAULT_KEY_BITLENGTH, &login);
        let pub_key = KeyPair::pub_to_pem(&keypair);
        let priv_key = KeyPair::priv_to_pem(&keypair, &password);

        if pub_key.is_empty() || priv_key.is_empty() {
            Err(format!("{}", ConfigError::KeyGenerationError))
        } else {
            Ok((pub_key, priv_key))
        }
    })
    .await;

    match result {
        Ok(ok) => ok,
        Err(_) => Err("Erreur interne de génération".into()),
    }
}

#[tauri::command]
async fn send_gpg_key(
    tokentypestring: String,
    gitlab_token: String,
    pub_key: String,
) -> Result<String, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        let tokentype = match tokentypestring.as_str() {
            "classic" => TokenType::Classic,
            "oauth" => TokenType::OAuth,
            _ => return Err(format!("{}", ConfigError::InternalError)),
        };
        let gitlab_client = GitlabClient::new(gitlab_token, tokentype);
        match gitlab_client.add_gpg_key(&pub_key) {
            Ok(_) => Ok("".to_string()),
            Err(e) => Err(format!("{}", e)),
        }
    })
    .await;

    match result {
        Ok(r) => r,
        Err(_) => Err("Erreur lors de l’envoi de la clé".into()),
    }
}

#[tauri::command]
async fn get_gitlab_oauth_token() -> Result<serde_json::Value, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        match GitlabClient::get_token() {
            Ok(token) => Ok(serde_json::json!({ "token": token })),
            Err(e) => Err(format!("{}", e)),
        }
    })
    .await;
    match result {
        Ok(r) => r,
        Err(_) => Err("Erreur interne OAuth".into()),
    }
}

#[tauri::command]
async fn get_login(
    token: String,
    tokentypestring: String,
) -> Result<String, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        let tokentype = match tokentypestring.as_str() {
            "classic" => TokenType::Classic,
            "oauth" => TokenType::OAuth,
            _ => return Err(format!("{}", ConfigError::InternalError)),
        };
        // enlever les commentaires quand la fonction sera implémentée
        /*
        let gitlab_client = GitlabClient::new(token, tokentype);
        match gitlab_client.get_login_from_token(&token, &tokentype) {
            Ok(login) => Ok(login),
            Err(e) => Err(format!("{}", e)),
        }
        */
        Ok("".to_string())
    })
    .await;

    match result {
        Ok(r) => r,
        Err(_) => Err("Erreur lors de la récupération du login".into()),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            check_config_values,
            get_gitlab_oauth_token,
            load_config_from_file,
            save_config_to_file,
            keypair_generation,
            send_gpg_key,
            get_login
        ])
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
