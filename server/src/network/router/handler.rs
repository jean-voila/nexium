use crate::blockchain::{blockchain::Blockchain, cache::cache::Cache};

use super::{
    http::{request::Request, response::Response, status::Status},
    routes::{check_nexium, get_balance, get_transactions, new_transaction},
};
use nexium::rsa::KeyPair;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub async fn handler(
    stream: TcpStream,
    cache: Arc<Mutex<Cache>>,
    blockchain: Arc<Mutex<Blockchain>>,
    login: String,
    key: KeyPair,
) {
    println!("New connection: {}", stream.peer_addr().unwrap());

    let req = match Request::from_stream(stream).await {
        Ok(r) => r,
        Err((e, stream)) => {
            let res = Response::new(Status::BadRequest, e);
            let _ = Request::_send(stream, &res);
            return;
        }
    };

    println!("method: {}", req.method);
    println!("path: {}", req.path);
    println!("path_query: {}", req.path_query);
    println!("query:");
    for (key, val) in req.query.iter() {
        println!("'{key}': '{val}'");
    }
    println!("------------------");
    println!("header:");
    for (key, val) in req.headers.iter() {
        println!("'{key}': '{val}'");
    }
    println!("------------------");
    println!("body length: {}", req.body.len());
    println!("body: {}", req.body);
    println!("------------------");

    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/nexium") => {
            check_nexium::handler(req, cache, login, key).await;
        }
        (method, path) if method == "GET" && path.starts_with("/balance/") => {
            get_balance::handler(req, cache, blockchain).await;
        }
        (method, path)
            if method == "GET" && path.starts_with("/transactions/") =>
        {
            get_transactions::handler(req, cache, blockchain).await;
        }
        ("POST", "/new_transaction") => {
            new_transaction::handler(req, cache, blockchain, key).await;
        }
        _ => {
            let res = Response::new(Status::NotFound, "");
            let _ = req.send(&res).await;
        }
    };
}
