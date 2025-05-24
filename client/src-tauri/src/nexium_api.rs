use super::config::*;
use json;
use log::kv::Key;
use nexium::blockchain::transaction::*;
use nexium::defaults::*;
use nexium::gitlab;
use nexium::gitlab::*;
use nexium::rsa::*;
use num_bigint::BigUint;
use reqwest::blocking::Client;
use reqwest::header::PUBLIC_KEY_PINS_REPORT_ONLY;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::{collections::HashMap, str::FromStr};

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
    NoServerResponse,
    InvalidResponseFromServer,
    InvalidJsonResponse,
    NoServerLogin,
    NoServerSigSample,
    NoServerGpgKeys,
    InvalidSigSample,
    NoBalanceField,
    InvalidBalanceFormat,
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

pub fn get_server_pub_key(config: Config) -> Result<String, String> {
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
            SIG_SAMPLE.as_bytes().to_vec(),
            &sig_sample_biguint,
        ) {
            Ok(res) => {
                if res {
                    return Ok(key);
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

    dbg!(&balance_str);
    let parts: Vec<&str> = balance_str.split('.').collect();
    let part0 = match parts.get(0) {
        Some(p) => p.to_string(),
        None => "0".to_string(),
    };
    let part1 = match parts.get(1) {
        Some(p) => p.to_string(),
        None => "".to_string(),
    };

    if part1.len() > 2 {
        return Err(NexiumAPIError::InvalidBalanceFormat.to_string());
    }

    Ok((part0, part1))
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
