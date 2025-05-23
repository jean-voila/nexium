use super::config::*;
use nexium::blockchain::transaction::*;
use nexium::defaults::*;
use nexium::gitlab::*;
use nexium::rsa::*;
use reqwest::header::PUBLIC_KEY_PINS_REPORT_ONLY;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionInOrOout {
    IN,
    OUT,
}

#[derive(Debug)]
pub enum NexiumAPIError {
    UnknownError,
    InvalidPrivateKeyOrPassword,
    NoServerPublicKey,
}

impl fmt::Display for NexiumAPIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            NexiumAPIError::UnknownError => "Erreur inconnue.",
            NexiumAPIError::InvalidPrivateKeyOrPassword => {
                "Clé privée ou mot de passe invalide."
            }
            NexiumAPIError::NoServerPublicKey => {
                "Impossible de récupérer la clé publique du serveur."
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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassicTransactionReceived {
    pub receiver: String,
    pub emitter: String,
    pub description: String,
    pub amount: String,
    pub date: String,
    pub in_or_out: TransactionInOrOout,
}

fn build_headers(
    config: &Config,
) -> Result<reqwest::header::HeaderMap, String> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Login",
        reqwest::header::HeaderValue::from_str(&config.user_login).unwrap(),
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

pub fn get_server_pub_key(config: Config) -> Result<String, String> {
    return Ok("".to_string());
    let headers = build_headers(&config);
    let url = build_url(&config, "/nexium");
    /*
    let client = Client::new();
    let response = match client.get(&url).headers(headers?).send() {
        Ok(resp) => resp,
        Err(_) => return Err(NexiumAPIError::UnknownError.to_string()),
    };

    let json: serde_json::Value = match response.json() {
        Ok(j) => j,
        Err(_) => return Err(NexiumAPIError::UnknownError.to_string()),
    };

    let login = match json.get("login").and_then(|v| v.as_str()) {
        Some(l) => l.to_string(),
        None => return Err(NexiumAPIError::UnknownError.to_string()),
    };

    let sig_sample = match json.get("sigSample").and_then(|v| v.as_str()) {
        Some(s) => s.to_string(),
        None => return Err(NexiumAPIError::UnknownError.to_string()),
    };

    let gpg_keys = match nexium::gitlab::get_gpg_keys(&config, &login) {
        Ok(keys) => keys,
        Err(_) => return Err(NexiumAPIError::UnknownError.to_string()),
    };

    for key in gpg_keys {
        if let Ok(pub_key) = GPGPublicKey::from_armored(&key) {
            if pub_key.verify(SIG_SAMPLE, &sig_sample).unwrap_or(false) {
                return Ok(key);
            }
        }
    }

    Err(NexiumAPIError::NoServerPublicKey.to_string())*/
    todo!();
}

pub fn send_transaction(
    pub_key: String,
    transaction: ClassicTransactionSent,
    config: Config,
) -> Result<(), String> {
    let headers = build_headers(&config);
    todo!();
}

pub fn get_balance(
    pub_key: String,
    login: String,
    config: Config,
) -> Result<(String, String), String> {
    let headers = build_headers(&config);
    todo!();
}

pub fn get_transactions(
    pub_key: String,
    config: Config,
    login: String,
    n: String,
) -> Result<Vec<ClassicTransactionReceived>, String> {
    let headers = build_headers(&config);
    todo!();
}
