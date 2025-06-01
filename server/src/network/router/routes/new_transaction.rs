use std::sync::Arc;

use nexium::{blockchain::transaction::Transaction, rsa::KeyPair};
use tokio::sync::Mutex;

use crate::{
    blockchain::{blockchain::Blockchain, cache::cache::Cache},
    network::http::{request::Request, response::Response, status::Status},
};

pub async fn handler(
    req: Request,
    mut res: Response,
    cache: Arc<Mutex<Cache>>,
    blockchain: Arc<Mutex<Blockchain>>,
    key: KeyPair,
) {
    let data = match key.decrypt_split(&req.body) {
        Ok(res) => res,
        Err(_) => {
            res.status = Status::BadRequest;
            res.send(b"Failed to decrypt request body").await;
            return;
        }
    };

    let tr: Transaction = match serde_json::from_str(&data) {
        Ok(obj) => obj,
        Err(e) => {
            res.status = Status::BadRequest;
            let msg = format!("Failed to parse transaction: {}", e.to_string());
            res.send(msg.as_bytes()).await;
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
            res.status = Status::BadRequest;
            res.send(b"Invalid key").await;
            return;
        }
    };

    match key.check_signature(&message, &tr.signature) {
        Ok(check) => {
            if !check {
                res.status = Status::BadRequest;
                res.send(b"Invalid signature").await;
                return;
            }
        }
        Err(_) => {
            res.status = Status::BadRequest;
            res.send(b"Failed to check signature").await;
            return;
        }
    }

    res.status = Status::Ok;
    res.send(b"").await;

    blockchain.lock().await.add_transaction(tr).await;
}
