use super::config::*;

use chrono::DateTime;

use json;
use nexium::blockchain::transaction::*;
use nexium::blockchain::transaction_data::*;
use nexium::defaults::*;
use nexium::gitlab::*;
use nexium::rsa::*;
use num_bigint::BigUint;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum NexiumAPIError {
    UnknownError,
    NoServerPublicKey,
    NoServerResponse,
    InvalidResponseFromServer,
    InvalidJsonResponse,
    NoServerLogin,
    NoServerSigSample,
    NoServerGpgKeys,
    InvalidSigSample,
    NoBalanceField,
    InvalidBalanceFormat,
    NegativeOrZeroAmount,
    InvalidTransactionAmount,
    InvalidFees,
}

impl fmt::Display for NexiumAPIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            NexiumAPIError::UnknownError => "Erreur inconnue.",

            NexiumAPIError::NoServerPublicKey => {
                "Impossible de récupérer la clé publique du serveur."
            }
            NexiumAPIError::NoServerResponse => "Aucune réponse du serveur.",
            NexiumAPIError::InvalidResponseFromServer => {
                "Réponse invalide du serveur"
            }
            NexiumAPIError::InvalidJsonResponse => "Réponse JSON invalide.",
            NexiumAPIError::NoServerLogin => {
                "Impossible de récupérer le login du serveur."
            }
            NexiumAPIError::NoServerSigSample => {
                "Impossible de récupérer le sample de signature du serveur."
            }
            NexiumAPIError::NoServerGpgKeys => {
                "Impossible de récupérer les clés GPG du serveur."
            }
            NexiumAPIError::InvalidSigSample => "Sample de signature invalide.",
            NexiumAPIError::NoBalanceField => {
                "Impossible de récupérer le champ de solde."
            }
            NexiumAPIError::InvalidBalanceFormat => "Format de solde invalide.",
            NexiumAPIError::NegativeOrZeroAmount => {
                "Le montant doit être supérieur à zéro."
            }
            NexiumAPIError::InvalidTransactionAmount => {
                "Montant de transaction invalide."
            }
            NexiumAPIError::InvalidFees => {
                "Les frais de transaction doivent être un entier positif."
            }
        };
        write!(f, "{}", msg)
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassicTransactionSent {
    pub receiver: String,
    pub amount: String,
    pub description: String,
    pub fees: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ClassicTransactionReceived {
    pub receiver: String,
    pub emitter: String,
    pub description: String,
    pub amount: String,
    pub date: String,
    pub inorout: String,
}

fn build_headers(
    config: &Config,
) -> Result<reqwest::header::HeaderMap, String> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Login",
        match reqwest::header::HeaderValue::from_str(&config.user_login) {
            Ok(l) => l,
            Err(e) => {
                return Err(e.to_string());
            }
        },
    );

    let private_key = match KeyPair::priv_from_pem(
        &config.priv_key,
        &config.password,
        &config.user_login,
    ) {
        Ok(key) => key,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let signature = match private_key.sign(SIG_SAMPLE) {
        Ok(sig) => sig,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    headers.insert(
        "Sig-Sample",
        match reqwest::header::HeaderValue::from_str(&signature.to_string()) {
            Ok(sig) => sig,
            Err(e) => {
                return Err(e.to_string());
            }
        },
    );

    headers.insert(
        "Content-Type",
        reqwest::header::HeaderValue::from_static("text/plain"),
    );
    return Ok(headers);
}

fn build_url(config: &Config, endpoint: &str) -> String {
    format!(
        "http://{}:{}/{}",
        config.server_address,
        config.port,
        endpoint.trim_start_matches('/')
    )
}

pub fn get_server_key_login(
    config: Config,
) -> Result<(String, String), String> {
    let headers = match build_headers(&config) {
        Ok(h) => h,
        Err(e) => return Err(e.to_string()),
    };

    let url = build_url(&config, "/nexium");

    let client = Client::new();
    let response = match client.get(&url).headers(headers).send() {
        Ok(r) => r,
        Err(_) => return Err(NexiumAPIError::NoServerResponse.to_string()),
    };

    let client_key = match KeyPair::priv_from_pem(
        &config.priv_key,
        &config.password,
        &config.user_login,
    ) {
        Ok(key) => key,
        Err(e) => return Err(e.to_string()),
    };

    match response.status() {
        reqwest::StatusCode::OK => {}
        e => {
            return Err(format!(
                "{}: {}",
                NexiumAPIError::InvalidResponseFromServer.to_string(),
                e.to_string()
            ));
        }
    }

    let response_text = match response.text() {
        Ok(t) => t,
        Err(_) => return Err(NexiumAPIError::NoServerResponse.to_string()),
    };

    let decrypted_response = match client_key.decrypt_split(&response_text) {
        Ok(d) => d,
        Err(e) => return Err(e.to_string()),
    };

    let json = match json::parse(&decrypted_response) {
        Ok(j) => j,
        Err(_) => return Err(NexiumAPIError::InvalidJsonResponse.to_string()),
    };

    let server_login = match json["login"].as_str() {
        Some(l) => l.to_string(),
        None => return Err(NexiumAPIError::NoServerLogin.to_string()),
    };

    let sig_sample = match json["sigSample"].as_str() {
        Some(s) => s.to_string(),
        None => return Err(NexiumAPIError::NoServerSigSample.to_string()),
    };

    let gitlab_client =
        GitlabClient::new(config.gitlab_token, config.gitlab_token_type);

    let gpg_keys = match gitlab_client.get_gpg_keys(&server_login) {
        Ok(keys) => keys,
        Err(_) => return Err(NexiumAPIError::NoServerGpgKeys.to_string()),
    };

    for key in gpg_keys {
        let server_key = match KeyPair::pub_from_pem(&key, &server_login) {
            Ok(k) => k,
            Err(e) => return Err(e.to_string()),
        };

        let sig_sample_biguint = match BigUint::from_str(&sig_sample) {
            Ok(b) => b,
            Err(_) => return Err(NexiumAPIError::InvalidSigSample.to_string()),
        };
        match server_key.check_signature(
            &SIG_SAMPLE.as_bytes().to_vec(),
            &sig_sample_biguint,
        ) {
            Ok(res) => {
                if res {
                    return Ok((key, server_login));
                }
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    return Err(NexiumAPIError::NoServerPublicKey.to_string());
}

pub fn send_transaction(
    server_pubkey: String,
    transaction: ClassicTransactionSent,
    config: Config,
) -> Result<(), String> {
    let headers = build_headers(&config);

    let client_key = match KeyPair::priv_from_pem(
        &config.priv_key,
        &config.password,
        &config.user_login,
    ) {
        Ok(key) => key,
        Err(e) => return Err(e.to_string()),
    };

    let amount = match transaction.amount.parse::<f32>() {
        Ok(n) => {
            if n <= 0.0 {
                return Err(NexiumAPIError::NegativeOrZeroAmount.to_string());
            }
            n
        }
        Err(_) => {
            return Err(NexiumAPIError::InvalidTransactionAmount.to_string())
        }
    };

    let fees = match transaction.fees.parse::<u16>() {
        Ok(n) => n,
        Err(_) => return Err(NexiumAPIError::InvalidFees.to_string()),
    };

    let transaction = match Transaction::new_classic(
        &transaction.receiver,
        amount,
        &transaction.description,
        fees,
        &config.user_login,
        &client_key,
    ) {
        Ok(t) => t,
        Err(e) => return Err(e.to_string()),
    };

    let body = match serde_json::to_string(&transaction) {
        Ok(t) => t,
        Err(_) => return Err(NexiumAPIError::UnknownError.to_string()),
    };

    let server_pubkey =
        match KeyPair::pub_from_pem(&server_pubkey, &config.server_login) {
            Ok(k) => k,
            Err(e) => return Err(e.to_string()),
        };

    let encrypted_body = match server_pubkey.crypt_split(&body) {
        Ok(e) => e,
        Err(e) => return Err(e.to_string()),
    };

    let url = build_url(&config, "/new_transaction");
    let client = Client::new();
    let response = match client
        .post(&url)
        .headers(headers.unwrap_or_default())
        .body(encrypted_body)
        .send()
    {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    if !response.status().is_success() {
        return Err(format!(
            "{}: {}",
            NexiumAPIError::InvalidResponseFromServer.to_string(),
            response.status()
        ));
    };
    return Ok(());
}

pub fn get_balance(
    login: String,
    config: Config,
) -> Result<(String, String), String> {
    let headers = match build_headers(&config) {
        Ok(h) => h,
        Err(e) => return Err(e),
    };

    let url = build_url(&config, &format!("/balance/{}", login));

    let client = Client::new();
    let response = match client.get(&url).headers(headers).send() {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    if !response.status().is_success() {
        return Err(format!(
            "{}: {}",
            NexiumAPIError::InvalidResponseFromServer.to_string(),
            response.status()
        ));
    }

    let response_text = match response.text() {
        Ok(t) => t,
        Err(_) => return Err(NexiumAPIError::NoServerResponse.to_string()),
    };
    let client_key = match KeyPair::priv_from_pem(
        &config.priv_key,
        &config.password,
        &config.user_login,
    ) {
        Ok(key) => key,
        Err(e) => return Err(e.to_string()),
    };

    let uncrypted_response = match client_key.decrypt_split(&response_text) {
        Ok(d) => d,
        Err(e) => return Err(e.to_string()),
    };

    let json = match json::parse(&uncrypted_response) {
        Ok(j) => j,
        Err(_) => return Err(NexiumAPIError::InvalidJsonResponse.to_string()),
    };

    let balance_str = match json["balance"].as_str() {
        Some(b) => b.to_string(),
        None => {
            if let Some(num) = json["balance"].as_i64() {
                num.to_string()
            } else if let Some(num) = json["balance"].as_f64() {
                num.to_string()
            } else {
                return Err(NexiumAPIError::NoBalanceField.to_string());
            }
        }
    };

    let parts: Vec<&str> = balance_str.split('.').collect();

    let part0 = match parts.get(0) {
        Some(p) => p.to_string(),
        None => "0".to_string(),
    };
    let part1 = match parts.get(1) {
        Some(p) => p.to_string(),
        None => "0".to_string(),
    };

    if parts.len() > 2 {
        return Err(NexiumAPIError::InvalidBalanceFormat.to_string());
    }

    let part1 = if part1.len() > 2 {
        part1.chars().take(2).collect::<String>()
    } else {
        part1
    };

    Ok((part0, part1))
}

pub fn get_transactions(
    config: Config,
    login: String,
    n: String,
) -> Result<Vec<ClassicTransactionReceived>, String> {
    let headers = match build_headers(&config) {
        Ok(h) => h,
        Err(e) => return Err(e),
    };

    let url = build_url(&config, &format!("/transactions/{}?n={}", login, n));

    let client = Client::new();
    let response = match client.get(&url).headers(headers).send() {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    if !response.status().is_success() {
        return Err(format!(
            "{}: {}",
            NexiumAPIError::InvalidResponseFromServer.to_string(),
            response.status()
        ));
    }

    let response_text = match response.text() {
        Ok(t) => t,
        Err(_) => return Err(NexiumAPIError::NoServerResponse.to_string()),
    };
    let client_key = match KeyPair::priv_from_pem(
        &config.priv_key,
        &config.password,
        &config.user_login,
    ) {
        Ok(key) => key,
        Err(e) => return Err(e.to_string()),
    };

    let uncrypted_response = match client_key.decrypt_split(&response_text) {
        Ok(d) => d,
        Err(e) => return Err(e.to_string()),
    };

    let json = match json::parse(&uncrypted_response) {
        Ok(j) => j,
        Err(_) => return Err(NexiumAPIError::InvalidJsonResponse.to_string()),
    };

    let mut transactions: Vec<ClassicTransactionReceived> = vec![];

    for tr_json in json.members() {
        let tr_str = match tr_json.as_str() {
            Some(s) => s,
            None => continue,
        };

        let tr: Transaction = match serde_json::from_str(tr_str) {
            Ok(t) => t,
            Err(_) => {
                return Err(NexiumAPIError::InvalidJsonResponse.to_string());
            }
        };
        let data = match Transaction::get_data(&tr) {
            Ok(d) => d,
            Err(_) => {
                return Err(NexiumAPIError::InvalidJsonResponse.to_string());
            }
        };

        match data {
            TransactionData::ClassicTransaction {
                receiver,
                amount,
                has_description,
                description,
            } => {
                let receiver = String::from_utf8_lossy(&receiver)
                    .to_string()
                    .trim_end_matches('\0')
                    .to_string();

                let emitter =
                    String::from_utf8_lossy(&tr.header.emitter).to_string();
                let description = if has_description {
                    String::from_utf8_lossy(&description).to_string()
                } else {
                    "".to_string()
                };

                let in_or_out = if receiver == login {
                    "IN".to_string()
                } else {
                    "OUT".to_string()
                };

                let datetime: Option<DateTime<chrono::Utc>> =
                    DateTime::from_timestamp(tr.header.timestamp as i64, 0);

                let formatted_date = match datetime {
                    Some(dt) => dt.format("%d/%m/%Y %H:%M").to_string(),
                    None => "Inconnue".to_string(),
                };

                let transaction = ClassicTransactionReceived {
                    receiver: receiver.clone(),
                    emitter: emitter.clone(),
                    description,
                    amount: amount.to_string(),
                    date: formatted_date,
                    inorout: in_or_out,
                };
                transactions.push(transaction);
            }
            _ => {
                continue; // Skip non-classic transactions
            }
        }
    }

    Ok(transactions)
}
