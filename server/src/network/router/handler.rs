use crate::{
    blockchain::blockchain::Blockchain, network::http::method::Method,
};

use super::{
    super::http::{request::Request, response::Response, status::Status},
    routes::{check_nexium, get_balance, get_transactions, new_transaction},
};
use nexium::{gitlab::GitlabClient, rsa::KeyPair};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub async fn handler(
    mut stream: TcpStream,
    gitlab: Arc<Mutex<GitlabClient>>,
    blockchain: Arc<Mutex<Blockchain>>,
    login: Arc<String>,
    key: Arc<KeyPair>,
) -> Result<(), String> {
    println!("New connection: {}", stream.peer_addr().unwrap());

    let req = Request::from_stream(&mut stream)
        .await
        .map_err(|e| format!("Failed to parse request: {}", e))?;
    let mut res = Response::new(stream);

    match (&req.method, req.path.as_str()) {
        (Method::Get, "/nexium") => {
            check_nexium::handler(req, res, gitlab, login, key).await
        }
        (method, path)
            if *method == Method::Get && path.starts_with("/balance/") =>
        {
            get_balance::handler(req, res, gitlab, blockchain).await
        }
        (method, path)
            if *method == Method::Get && path.starts_with("/transactions/") =>
        {
            get_transactions::handler(req, res, gitlab, blockchain).await
        }
        (Method::Post, "/new_transaction") => {
            new_transaction::handler(req, res, gitlab, blockchain, key).await
        }
        _ => {
            res.status = Status::NotFound;
            res.send_empty().await
        }
    }
    .map_err(|e| format!("Failed to send response: {}", e))
}
