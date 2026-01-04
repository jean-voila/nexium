// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod config;
mod contacts;
mod invoice;
mod nexium_api;

use config::Config;
use config::ConfigError;
use invoice::*;

use nexium_api::*;

use nexium::{
    blockchain::consts::estimate_classic_transaction_fee, defaults::*,
    gitlab::*, login::*, rsa::*,
};
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
async fn load_config() -> Result<Option<Config>, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        Config::load()
    })
    .await;

    match result {
        Ok(config) => Ok(config),
        Err(_) => Err(ConfigError::FileReadError.to_string()),
    }
}

#[tauri::command]
async fn save_config(config: Config) -> Result<(), String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        config.save()
    })
    .await;

    match result {
        Ok(r) => r.map_err(|e| e.to_string()),
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

/// Calculate the estimated fee cost for a transaction
/// Returns the fee cost in NEX as a formatted string
#[tauri::command]
async fn calculate_transaction_fee(
    fees: String,
    has_description: bool,
) -> Result<String, String> {
    let fees_per_byte = match fees.parse::<u16>() {
        Ok(n) => n,
        Err(_) => return Ok("0".to_string()),
    };

    let fee_cost = estimate_classic_transaction_fee(fees_per_byte, has_description);
    Ok(format!("{:.6}", fee_cost))
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

        // Parse and validate fees
        let fees = match transaction.fees.parse::<u16>() {
            Ok(n) => n,
            Err(_) => return Err(NexiumAPIError::InvalidFees.to_string()),
        };

        // Calculate the estimated fee cost based on transaction type
        let has_description = !transaction.description.is_empty();
        let fee_cost = estimate_classic_transaction_fee(fees, has_description);

        // Total cost = amount + fees
        let total_cost = amount + fee_cost;

        let available_balance = match nexium_api::get_balance(
            config.user_login.clone(),
            config.clone(),
        ) {
            Ok((int, dec)) => {
                format!("{}.{}", int, dec).parse::<f64>().unwrap_or(0.0)
            }
            Err(_) => return Err(NexiumAPIError::BalanceFetchError.to_string()),
        };

        if total_cost > available_balance {
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
async fn get_peers(config: Config) -> Result<Vec<nexium_api::PeerInfo>, String> {
    let result = tauri::async_runtime::spawn_blocking(move || {
        match nexium_api::get_peers(config) {
            Ok(peers) => Ok(peers),
            Err(e) => Err(e),
        }
    })
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

/// Try to find a working server from the cached peer list
#[tauri::command]
async fn find_working_server(config: Config) -> Result<(String, String, Config), String> {
    // First try the current server
    let config_clone = config.clone();
    let current_result = tauri::async_runtime::spawn_blocking(move || {
        nexium_api::get_server_key_login(config_clone)
    })
    .await;
    
    if let Ok(Ok((pub_key, login))) = current_result {
        let mut updated_config = config.clone();
        updated_config.server_login = login.clone();
        return Ok((pub_key, login, updated_config));
    }
    
    // Current server failed, try to get cached peers
    // We need to try each peer until one works
    let peers = get_cached_peers();
    
    for peer in peers {
        // Skip current server
        if peer.address == config.server_address && peer.port.to_string() == config.port {
            continue;
        }
        
        let mut test_config = config.clone();
        test_config.server_address = peer.address.clone();
        test_config.port = peer.port.to_string();
        
        let test_config_clone = test_config.clone();
        let result = tauri::async_runtime::spawn_blocking(move || {
            nexium_api::get_server_key_login(test_config_clone)
        })
        .await;
        
        if let Ok(Ok((pub_key, login))) = result {
            test_config.server_login = login.clone();
            println!("Failover: switched to server {}:{}", peer.address, peer.port);
            return Ok((pub_key, login, test_config));
        }
    }
    
    Err("No available servers found".to_string())
}

/// Get cached peers from local storage
fn get_cached_peers() -> Vec<nexium_api::PeerInfo> {
    let path = get_peers_cache_path();
    if !path.exists() {
        return vec![];
    }
    
    match std::fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => vec![],
    }
}

/// Save peers to local cache
fn save_peers_cache(peers: &[nexium_api::PeerInfo]) {
    let path = get_peers_cache_path();
    if let Ok(content) = serde_json::to_string(peers) {
        let _ = std::fs::write(&path, content);
    }
}

fn get_peers_cache_path() -> std::path::PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    path.push("nexium");
    let _ = std::fs::create_dir_all(&path);
    path.push("peers_cache.json");
    path
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            check_config_values,
            get_gitlab_oauth_token,
            load_config_from_file,
            save_config_to_file,
            load_config,
            save_config,
            keypair_generation,
            send_gpg_key,
            get_login,
            save_facture_to_file,
            check_invoice_values,
            get_names_from_login,
            load_invoice_from_file,
            get_invoice_extension,
            calculate_transaction_fee,
            get_balance,
            send_transaction,
            get_transactions,
            is_testnet,
            get_server_infos,
            write_key_to_file,
            read_key_from_file,
            check_send_transaction,
            search_first_users,
            contacts::get_contacts,
            contacts::add_contact,
            contacts::update_contact,
            contacts::remove_contact,
            contacts::search_contacts,
            contacts::get_favorite_contacts,
            contacts::get_recent_contacts,
            contacts::mark_contact_used,
            get_user_stats,
            get_peers,
            check_peer_status,
            try_connect_to_server,
            find_working_server,
        ])
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
