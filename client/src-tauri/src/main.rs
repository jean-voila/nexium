// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod config;
mod contacts;
mod core;
mod invoice;
mod nexium_api;
mod types;
use commands::*;
use config::{Config, ConfigError};
use invoice::*;
use nexium::{defaults::*, gitlab::*, rsa::*};
use nexium_api::*;
use std::path::Path;

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

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            get_constants::get_constants,
            check_config_values::check_config_values,
            get_gitlab_oauth_token::get_gitlab_oauth_token,
            load_config_from_file::load_config_from_file,
            save_config_to_file::save_config_to_file,
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
            get_peers::get_peers,
            check_peer_status::check_peer_status,
            try_connect_to_server::try_connect_to_server,
            find_working_server::find_working_server,
        ])
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
