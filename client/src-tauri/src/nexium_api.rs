use nexium::blockchain::transaction::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub enum RequestMethod {
    GET,
    POST,
}

pub struct RequestSpec {
    pub path: String,
    pub method: RequestMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NexiumConfig {
    login: String,
    server_url: String,
    server_port: String,
}

pub enum NexiumEndPoint {
    Nexium,
    Balance,
    Transactions,
    NewTransaction,
}

impl NexiumEndPoint {
    pub fn request_spec(&self, login: &str, n: Option<usize>) -> RequestSpec {
        match self {
            NexiumEndPoint::Nexium => RequestSpec {
                path: "/nexium".to_string(),
                method: RequestMethod::GET,
            },
            NexiumEndPoint::Balance => RequestSpec {
                path: format!("/balance/{}", login),
                method: RequestMethod::GET,
            },
            NexiumEndPoint::Transactions => {
                let mut path = format!("/transactions/{}", login);
                if let Some(count) = n {
                    path.push_str(&format!("?n={}", count));
                }
                RequestSpec {
                    path,
                    method: RequestMethod::GET,
                }
            }
            NexiumEndPoint::NewTransaction => RequestSpec {
                path: "/new_transaction".to_string(),
                method: RequestMethod::POST,
            },
        }
    }
}
