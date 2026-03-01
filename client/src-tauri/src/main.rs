// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod config;
mod contacts;
mod core;
mod invoice;
mod nexium_api;
mod types;
use crate::core::peer_cache::save_peers_cache;
use commands::*;
use config::{Config, ConfigError};
use invoice::*;
use nexium::{defaults::*, gitlab::*, rsa::*};
use nexium_api::*;
use std::path::Path;

#[tauri::command]
async fn load_config_from_file(path_string: String) -> Result<Config, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        if Path::new(&path_string).exists() == false {
            return Err(ConfigError::FileNotFound.to_string());
        }
        let path = Path::new(&path_string);
        match Config::from_file(path) {
            Ok(config) => Ok(config),
            Err(e) => Err(e),
        }
    })
    .await;

    match result {
        Ok(r) => match r {
            Ok(config) => Ok(config),
            Err(e) => Err(e.to_string()),
        },
        Err(_) => Err(ConfigError::FileReadError.to_string()),
    }
}

#[tauri::command]
async fn save_config_to_file(
    config: Config,
    path_string: String,
) -> Result<String, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        let path = Path::new(&path_string);

        match Config::to_file(&config, path) {
            Ok(_) => Ok("".to_string()),
            Err(e) => Err(e.to_string()),
        }
    })
    .await;

    match result {
        Ok(r) => r,
        Err(_) => Err(ConfigError::FileWriteError.to_string()),
    }
}

#[tauri::command]
async fn save_facture_to_file(
    invoice: Invoice,
    path_string: String,
) -> Result<String, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        let path = Path::new(&path_string);
        match Invoice::to_file(&invoice, path) {
            Ok(_) => Ok("".to_string()),
            Err(e) => Err(e.to_string()),
        }
    })
    .await;

    match result {
        Ok(r) => r,
        Err(_) => Err(ConfigError::FileWriteError.to_string()),
    }
}

#[tauri::command]
async fn check_invoice_values(invoice: Invoice) -> Result<String, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        match invoice.check_values() {
            Ok(_) => Ok("".to_string()),
            Err(e) => Err(e.to_string()),
        }
    })
    .await;

    match result {
        Ok(r) => r,
        Err(_) => Err("Thread panicked during invoice validation".to_string()),
    }
}

#[tauri::command]
async fn keypair_generation(
    login: String,
    password: String,
) -> Result<(String, String), String> {
    // Utilise spawn_blocking pour éviter de bloquer le thread principal
    let result = tauri::async_runtime::spawn_blocking(move || {
        let keypair = KeyPair::generate(KEYPAIR_BIT_SIZE, &login);
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
        Err(_) => Err(ConfigError::KeyGenerationError.to_string()),
    }
}

#[tauri::command]
async fn send_gpg_key(
    gitlab_token_type: TokenType,
    gitlab_token: String,
    pub_key: String,
) -> Result<String, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        let gitlab_client = GitlabClient::new(gitlab_token, gitlab_token_type);
        match gitlab_client.add_gpg_key(&pub_key) {
            Ok(_) => Ok("".to_string()),
            Err(e) => Err(e.to_string()),
        }
    })
    .await;

    match result {
        Ok(r) => r,
        Err(_) => Err(GitlabError::BadGPGFormat.to_string()),
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
        Err(_) => Err(GitlabError::NoWebBrowser.to_string()),
    }
}

#[tauri::command]
async fn get_login(
    gitlab_token: String,
    gitlab_token_type: TokenType,
) -> Result<String, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        let gitlab_client = GitlabClient::new(gitlab_token, gitlab_token_type);
        match gitlab_client.get_login() {
            Ok(login) => Ok(login),
            Err(e) => Err(e.to_string()),
        }
    })
    .await;

    match result {
        Ok(r) => match r {
            Ok(login) => Ok(login),
            Err(e) => Err(e),
        },
        Err(_) => Err(GitlabError::UserNotFound.to_string()),
    }
}

#[tauri::command]
async fn write_key_to_file(path: String, key: String) -> Result<(), String> {
    let result =
        tauri::async_runtime::spawn_blocking(move || -> Result<(), String> {
            let path = Path::new(&path);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            std::fs::write(path, key).map_err(|e| e.to_string())?;
            Ok(())
        })
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn read_key_from_file(path: String) -> Result<String, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        let path = Path::new(&path);
        if !path.exists() {
            return Err("Le fichier n'existe pas".to_string());
        }
        std::fs::read_to_string(path).map_err(|e| e.to_string())
    })
    .await;

    match result {
        Ok(content) => content,
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn get_user_stats(
    login: String,
    config: Config,
) -> Result<nexium_api::UserStats, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        match nexium_api::get_user_stats(login, config) {
            Ok(stats) => Ok(stats),
            Err(e) => Err(e),
        }
    })
    .await;
    match result {
        Ok(r) => r,
        Err(_) => Err(NexiumAPIError::UnknownError.to_string()),
    }
}

#[tauri::command]
async fn get_peers(
    config: Config,
) -> Result<Vec<nexium_api::PeerInfo>, String> {
    let result =
        tauri::async_runtime::spawn_blocking(
            move || match nexium_api::get_peers(config) {
                Ok(peers) => Ok(peers),
                Err(e) => Err(e),
            },
        )
        .await;
    match result {
        Ok(Ok(peers)) => {
            // Cache peers for failover
            save_peers_cache(&peers);
            Ok(peers)
        }
        Ok(Err(e)) => Err(e),
        Err(_) => Err(NexiumAPIError::UnknownError.to_string()),
    }
}

#[tauri::command]
async fn check_peer_status(address: String, port: u16) -> bool {
    let result = tauri::async_runtime::spawn_blocking(move || {
        nexium_api::check_peer_status(address, port)
    })
    .await;
    match result {
        Ok(r) => r,
        Err(_) => false,
    }
}

/// Try to connect to a specific server and return the server info if successful
#[tauri::command]
async fn try_connect_to_server(
    mut config: Config,
    address: String,
    port: u16,
) -> Result<(String, String, Config), String> {
    // Update config with new server
    config.server_address = address;
    config.port = port.to_string();

    let config_clone = config.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        nexium_api::get_server_key_login(config_clone)
    })
    .await;

    match result {
        Ok(Ok((pub_key, login))) => {
            config.server_login = login.clone();
            Ok((pub_key, login, config))
        }
        Ok(Err(e)) => Err(e),
        Err(_) => Err("Connection failed".to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            get_constants::get_constants,
            check_config_values::check_config_values,
            get_gitlab_oauth_token,
            load_config_from_file,
            save_config_to_file,
            load_config::load_config,
            save_config::save_config,
            keypair_generation,
            send_gpg_key,
            get_login,
            save_facture_to_file,
            check_invoice_values,
            get_names_from_login::get_names_from_login,
            load_invoice_from_file::load_invoice_from_file,
            calculate_transaction_fee::calculate_transaction_fee,
            get_balance::get_balance,
            send_transaction::send_transaction,
            get_transactions::get_transactions,
            get_server_infos::get_server_infos,
            write_key_to_file,
            read_key_from_file,
            check_send_transaction::check_send_transaction,
            search_first_users::search_first_users,
            contact_get::contact_get,
            contact_add::contact_add,
            contact_update::contact_update,
            contact_remove::contact_remove,
            contact_search::contact_search,
            contacts::get_recent_contacts,
            contact_mark_used::contact_mark_used,
            get_user_stats,
            get_peers,
            check_peer_status,
            try_connect_to_server,
            find_working_server::find_working_server,
        ])
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
