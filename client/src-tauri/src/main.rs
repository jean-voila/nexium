// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod config;
mod invoice;
mod nexium_api;

use config::Config;
use config::ConfigError;
use invoice::*;

use nexium_api::*;

use nexium::{defaults::*, gitlab::*, login::*, rsa::*};
// use sleep
use std::path::Path;

#[tauri::command]
async fn check_config_values(config: Config) -> Result<String, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        Config::check_values(&config)
            .map(|_| "".to_string())
            .map_err(|e| e.to_string())
    })
    .await;

    match result {
        Ok(r) => r,
        Err(_) => Err("Thread panicked during config validation".to_string()),
    }
}
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
async fn load_invoice_from_file(
    path_string: String,
) -> Result<Invoice, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        if Path::new(&path_string).exists() == false {
            return Err(InvoiceError::FileNotFound.to_string());
        }
        let path = Path::new(&path_string);
        match Invoice::from_file(path) {
            Ok(invoice) => Ok(invoice),
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
async fn get_names_from_login(
    login: String,
) -> Result<(String, String), String> {
    // Utilise login::new(login) et login::get_names() pour obtenir le nom et le prénom
    let login = Login::new(login);
    match login {
        Ok(login) => {
            let names = login.get_names();
            match names {
                Ok((first_name, last_name)) => {
                    return Ok((first_name, last_name))
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn get_invoice_extension() -> String {
    return NEXIUM_INVOICE_EXTENSION.to_string();
}

#[tauri::command]
async fn get_balance(
    login: String,
    config: Config,
) -> Result<(String, String), String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        match nexium_api::get_balance(login, config) {
            Ok((int, dec)) => Ok((int, dec)),
            Err(e) => Err(e),
        }
    })
    .await;
    match result {
        Ok(r) => match r {
            Ok(soldes) => Ok(soldes),
            Err(e) => Err(e),
        },
        Err(_) => Err(NexiumAPIError::UnknownError.to_string()),
    }
}

#[tauri::command]
async fn send_transaction(
    server_pubkey: String,
    config: Config,
    transaction: nexium_api::ClassicTransactionSent,
) -> Result<String, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        match nexium_api::send_transaction(server_pubkey, transaction, config) {
            Ok(_) => Ok("".to_string()),
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
async fn check_send_transaction(
    transaction: nexium_api::ClassicTransactionSent,
    config: Config,
) -> Result<(), String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        let amount = match transaction.amount.parse::<f64>() {
            Ok(amount) => {
                if amount <= 0.0 {
                    return Err(NexiumAPIError::InvalidAmount.to_string());
                }
                amount
            }
            Err(_) => return Err(NexiumAPIError::InvalidAmount.to_string()),
        };

        let available_balance = match nexium_api::get_balance(
            config.user_login.clone(),
            config.clone(),
        ) {
            Ok((int, dec)) => {
                format!("{}.{}", int, dec).parse::<f64>().unwrap_or(0.0)
            }
            Err(_) => return Err(NexiumAPIError::BalanceFetchError.to_string()),
        };

        if amount > available_balance {
            return Err(NexiumAPIError::InsufficientFunds.to_string());
        }

        // check if the receiver and the sender are not the same
        if transaction.receiver == config.user_login {
            return Err(NexiumAPIError::SenderAndReceiverSame.to_string());
        }

        let rec_login = match Login::new(transaction.receiver.clone()) {
            Ok(rec) => rec,
            Err(_) => return Err(NexiumAPIError::InvalidReceiver.to_string()),
        };

        match rec_login.get_names() {
            Ok((first_name, last_name)) => {
                if first_name.chars().count() < 2
                    || last_name.chars().count() < 2
                {
                    return Err(NexiumAPIError::InvalidReceiver.to_string());
                }
            }
            Err(_) => return Err(NexiumAPIError::InvalidReceiver.to_string()),
        };

        let gitlab_client =
            GitlabClient::new(config.gitlab_token, config.gitlab_token_type);

        match gitlab_client.check_user_existence(&transaction.receiver) {
            Ok(exists) => {
                if !exists {
                    return Err(NexiumAPIError::ReceiverNotFound.to_string());
                }
                return Ok(());
            }
            Err(_) => return Err(NexiumAPIError::ReceiverNotFound.to_string()),
        }
    })
    .await;
    match result {
        Ok(r) => r,
        Err(_) => Err(NexiumAPIError::UnknownError.to_string()),
    }
}

#[tauri::command]
async fn get_transactions(
    config: Config,
    login: String,
    n: String,
) -> Result<Vec<nexium_api::ClassicTransactionReceived>, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        match nexium_api::get_transactions(config, login, n) {
            Ok(transactions) => Ok(transactions),
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
async fn is_testnet() -> bool {
    return IS_TESTNET;
}

#[tauri::command]
async fn get_server_infos(config: Config) -> Result<(String, String), String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        match nexium_api::get_server_key_login(config) {
            Ok(resp) => Ok(resp),
            Err(e) => Err(e),
        }
    })
    .await;
    match result {
        Ok(r) => match r {
            Ok(resp) => Ok(resp),
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
async fn search_first_users(
    config: Config,
    search: String,
) -> Result<Vec<String>, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        let gitlab_client =
            GitlabClient::new(config.gitlab_token, config.gitlab_token_type);
        match gitlab_client.search_users(&search) {
            Ok(users) => Ok(users),
            Err(e) => Err(e.to_string()),
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
        .invoke_handler(tauri::generate_handler![
            check_config_values,
            get_gitlab_oauth_token,
            load_config_from_file,
            save_config_to_file,
            keypair_generation,
            send_gpg_key,
            get_login,
            save_facture_to_file,
            check_invoice_values,
            get_names_from_login,
            load_invoice_from_file,
            get_invoice_extension,
            get_balance,
            send_transaction,
            get_transactions,
            is_testnet,
            get_server_infos,
            write_key_to_file,
            read_key_from_file,
            check_send_transaction,
            search_first_users,
        ])
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
