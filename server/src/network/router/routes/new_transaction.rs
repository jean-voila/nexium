use std::sync::Arc;

use nexium::{blockchain::transaction::Transaction, rsa::KeyPair};
use tokio::sync::Mutex;

use crate::{
    blockchain::{blockchain::Blockchain, cache::cache::Cache},
    network::router::http::{
        request::Request, response::Response, status::Status,
    },
    peers::PeerList,
};

pub async fn handler(
    req: Request,
    cache: Arc<Mutex<Cache>>,
    blockchain: Arc<Mutex<Blockchain>>,
    peer_list: Arc<Mutex<PeerList>>,
    key: KeyPair,
    self_address: String,
    self_port: u16,
) {
    let data = match key.decrypt_split(&req.body) {
        Ok(res) => res,
        Err(_) => {
            let res = Response::new(Status::BadRequest, "Invalid data");
            let _ = req.send(&res).await;
            return;
        }
    };

    let tr: Transaction = match serde_json::from_str(&data) {
        Ok(obj) => obj,
        Err(e) => {
            let res = Response::new(Status::BadRequest, e.to_string());
            let _ = req.send(&res).await;
            return;
        }
    };

    // dbg!(&tr);
    let mut message = tr.header.to_buffer().to_vec();
    message.extend(&tr.data);

    let key = match cache
        .lock()
        .await
        .get_key(
            &tr.header.get_login(),
            &tr.signature.to_string(),
            Some(&message),
        )
        .await
    {
        Some(k) => k,
        None => {
            let res = Response::new(Status::BadRequest, "Invalid key");
            let _ = req.send(&res).await;
            return;
        }
    };

    match key.check_signature(&message, &tr.signature) {
        Ok(res) => {
            if !res {
                let res =
                    Response::new(Status::BadRequest, "Invalid signature");
                let _ = req.send(&res).await;
                return;
            }
        }
        Err(_) => {
            let res =
                Response::new(Status::BadRequest, "Failed to check signature");
            let _ = req.send(&res).await;
            return;
        }
    }

    let res = Response::new(Status::Ok, "");
    let _ = req.send(&res).await;

    // Broadcast transaction to all peers
    let peers = peer_list.lock().await;
    peers.broadcast_transaction(&tr, &self_address, self_port).await;
    drop(peers);

    // Add to local blockchain
    blockchain.lock().await.add_transaction(tr, peer_list.clone(), self_address, self_port).await;
}
