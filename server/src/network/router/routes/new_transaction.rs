use std::{ops::DerefMut, sync::Arc};

use nexium::{
    blockchain::transaction::Transaction, gitlab::GitlabClient, rsa::KeyPair,
};
use tokio::sync::Mutex;

use crate::{
    blockchain::blockchain::Blockchain,
    network::http::{request::Request, response::Response, status::Status},
};

pub async fn handler(
    req: Request,
    mut res: Response,
    gitlab: Arc<Mutex<GitlabClient>>,
    blockchain: Arc<Mutex<Blockchain>>,
    key: KeyPair,
) -> Result<(), std::io::Error> {
    let data = match key.decrypt_split(&req.body) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to decrypt request body: {}", e);
            res.status = Status::BadRequest;
            return res.send(b"Failed to decrypt request body").await;
        }
    };

    let tr: Transaction = match serde_json::from_str(&data) {
        Ok(obj) => obj,
        Err(e) => {
            eprintln!("Failed to parse transaction: {}", e);
            res.status = Status::BadRequest;
            return res.send(b"Failed to parse transaction").await;
        }
    };

    // dbg!(&tr);
    let mut message = tr.header.to_buffer().to_vec();
    message.extend(&tr.data);

    let key = match req.get_key(gitlab.lock().await.deref_mut()).await {
        Ok(data) => data,
        Err(e) => {
            res.status = Status::Unauthorized;
            return res.send(e.as_bytes()).await;
        }
    };

    match key.check_signature(&message, &tr.signature) {
        Ok(check) => {
            if !check {
                eprintln!("Invalid signature for transaction");
                res.status = Status::BadRequest;
                return res.send(b"Invalid signature").await;
            }
        }
        Err(e) => {
            eprintln!("Failed to check signature: {}", e);
            res.status = Status::BadRequest;
            return res.send(b"Failed to check signature").await;
        }
    }

    res.status = Status::Ok;
    res.send(b"").await?;

    blockchain
        .lock()
        .await
        .add_transaction(gitlab.lock().await.deref_mut(), tr)
        .await;
    Ok(())
}
