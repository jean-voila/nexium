use crate::blockchain::{blockchain::Blockchain, cache::cache::Cache};
use crate::peers::PeerList;

use super::{
    http::{request::Request, response::Response, status::Status},
    routes::{
        blockchain_download, blockchain_info, check_nexium, get_balance, get_peers, 
        get_transactions, get_user_stats, new_transaction, register_peer, sync_block, 
        sync_transaction
    },
};
use nexium::rsa::KeyPair;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub async fn handler(
    stream: TcpStream,
    cache: Arc<Mutex<Cache>>,
    blockchain: Arc<Mutex<Blockchain>>,
    peer_list: Arc<Mutex<PeerList>>,
    login: String,
    key: KeyPair,
    self_address: String,
    self_port: u16,
) {
    let req = match Request::from_stream(stream).await {
        Ok(r) => r,
        Err((e, stream)) => {
            let res = Response::new(Status::BadRequest, e);
            let _ = Request::_send(stream, &res);
            return;
        }
    };

    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/nexium") => {
            check_nexium::handler(req, cache, login, key).await;
        }
        ("GET", "/peers") => {
            get_peers::handler(req, peer_list).await;
        }
        ("GET", "/blockchain_info") => {
            blockchain_info::handler(req, blockchain).await;
        }
        ("GET", "/blockchain_download") => {
            blockchain_download::handler(req, blockchain).await;
        }
        ("POST", "/register_peer") => {
            register_peer::handler(req, peer_list, login, self_address.clone(), self_port).await;
        }
        ("POST", "/sync_transaction") => {
            sync_transaction::handler(req, blockchain, peer_list, self_address, self_port).await;
        }
        ("POST", "/sync_block") => {
            sync_block::handler(req, blockchain).await;
        }
        (method, path) if method == "GET" && path.starts_with("/balance/") => {
            get_balance::handler(req, cache, blockchain).await;
        }
        (method, path)
            if method == "GET" && path.starts_with("/transactions/") =>
        {
            get_transactions::handler(req, cache, blockchain).await;
        }
        (method, path)
            if method == "GET" && path.starts_with("/stats/") =>
        {
            get_user_stats::handler(req, cache, blockchain).await;
        }
        ("POST", "/new_transaction") => {
            new_transaction::handler(req, cache, blockchain, peer_list, key, self_address, self_port).await;
        }
        _ => {
            let res = Response::new(Status::NotFound, "");
            let _ = req.send(&res).await;
        }
    };
}
