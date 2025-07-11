use crate::{
    blockchain::{blockchain::Blockchain, structure::block::Block},
    network::http::{request::Request, response::Response, status::Status},
};
use nexium::{
    blockchain::transaction::Transaction, gitlab::GitlabClient, rsa::KeyPair,
};
use std::{ops::Deref, sync::Arc};
use tokio::sync::Mutex;

pub async fn handler(
    req: Request,
    mut res: Response,
    gitlab: Arc<Mutex<GitlabClient>>,
    blockchain: Arc<Mutex<Blockchain>>,
    key: Arc<KeyPair>,
) -> Result<(), std::io::Error> {
    let json = match key.decrypt_split(&req.body) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to decrypt request body: {}", e);
            res.status = Status::BadRequest;
            return res.send(b"Failed to decrypt request body").await;
        }
    };

    let tr: Transaction = match serde_json::from_str(&json) {
        Ok(obj) => obj,
        Err(e) => {
            eprintln!("Failed to parse transaction: {}", e);
            res.status = Status::BadRequest;
            return res.send(b"Failed to parse transaction").await;
        }
    };

    // dbg!(&tr);

    let key = match req.get_key(&gitlab).await {
        Ok(data) => data,
        Err(e) => {
            res.status = Status::Unauthorized;
            return res.send(e.as_bytes()).await;
        }
    };

    match tr.check_sign(&key) {
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
    // println!("Transaction received: {:?}", tr);

    let mempool_full = blockchain.lock().await.add_transaction(tr);

    if mempool_full {
        // run an other task to avoid blocking the current one
        let git: GitlabClient = gitlab.lock().await.deref().clone();

        tokio::spawn(async move {
            let mut b = blockchain.lock().await;
            println!("Mempool is full, creating a new block");

            let (trs, balances) = b.get_verified_transactions(git).await;

            if trs.is_empty() {
                println!("No valid transactions found: aborting block");
                return;
            }

            let h = b.last_hash;
            drop(b); // Drop to avoid deadlock when creating a new block

            let (block_buff, hash) = Block::create(h, trs);
            println!("Block created with hash: {}", hex::encode(&hash));

            let mut b = blockchain.lock().await;
            match b.write_block(&block_buff, hash) {
                Ok(_) => {
                    // println!("Block written successfully");
                    b.update_balances(balances);
                }
                Err(e) => {
                    eprintln!("Failed to write block: {}", e)
                }
            }
        });
    }

    Ok(())
}
