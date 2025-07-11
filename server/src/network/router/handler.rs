use crate::{
    blockchain::blockchain::Blockchain,
    network::http::method::Method,
    network::{
        http::{request::Request, response::Response, status::Status},
        router::routes::{
            check_nexium, get_balance, get_transactions, new_transaction,
        },
    },
};
use chrono::{Datelike, Timelike};
use colored::Colorize;
use nexium::{gitlab::GitlabClient, rsa::KeyPair};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

fn get_current_date() -> String {
    let now = chrono::offset::Local::now();
    format!(
        "[{:02}-{:02}-{} {:02}:{:02}:{:02}]",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    )
}

pub async fn handler(
    mut stream: TcpStream,
    gitlab: Arc<Mutex<GitlabClient>>,
    blockchain: Arc<Mutex<Blockchain>>,
    login: Arc<String>,
    key: Arc<KeyPair>,
) -> Result<(), String> {
    let addr = stream.peer_addr().unwrap();
    // println!("New connection: {}", addr);

    let req = Request::from_stream(&mut stream)
        .await
        .map_err(|e| format!("Failed to parse request: {}", e))?;
    let mut res = Response::new(stream);

    println!(
        "{} {} {} {}",
        get_current_date().truecolor(150, 150, 150),
        addr,
        req.method.as_str().green(),
        req.path
    );

    match (&req.method, req.path.as_str()) {
        (Method::Get, "/nexium") => {
            check_nexium::handler(req, res, gitlab, login, key).await
        }

        (Method::Get, path) if path.starts_with("/balance/") => {
            get_balance::handler(req, res, gitlab, blockchain).await
        }

        (Method::Get, path) if path.starts_with("/transactions/") => {
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
