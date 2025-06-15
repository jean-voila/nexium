use std::{ops::DerefMut, sync::Arc};

use nexium::{
    blockchain::{
        consts::TRANSACTION_RECEIVER, transaction_data::TransactionData,
    },
    gitlab::GitlabClient,
};
use tokio::sync::Mutex;

use crate::{
    blockchain::{
        blockchain::Blockchain,
        structure::consts::HEADER_PREVIOUS_BLOCK_HASH_SIZE,
    },
    network::http::{request::Request, response::Response, status::Status},
};

pub async fn handler(
    req: Request,
    mut res: Response,
    gitlab: Arc<Mutex<GitlabClient>>,
    blockchain: Arc<Mutex<Blockchain>>,
) -> Result<(), std::io::Error> {
    let sp: Vec<String> = req.path.split("/").map(|e| e.to_string()).collect();
    let login = &sp[2];

    if login.is_empty() {
        res.status = Status::BadRequest;
        return res.send(b"Missing user login").await;
    }
    println!("login: {}", login);

    let n = match req.query.get("n") {
        Some(n) => match n.parse::<usize>() {
            Ok(0) | Err(_) => 3,
            Ok(100..) => 100,
            Ok(x) => x,
        },
        None => 3,
    };
    println!("n: {n}");

    match gitlab.lock().await.check_user_existence_async(login).await {
        Ok(true) => {}
        Ok(false) => {
            res.status = Status::NotFound;
            return res.send(b"User not found").await;
        }
        Err(e) => {
            eprintln!("Failed to check user existence: {}", e);
            res.status = Status::InternalServerError;
            return res.send(b"Failed to check user existence").await;
        }
    }

    let key = match req.get_key(gitlab.lock().await.deref_mut()).await {
        Ok(data) => data,
        Err(e) => {
            res.status = Status::Unauthorized;
            return res.send(e.as_bytes()).await;
        }
    };

    let mut arr = json::array![];
    let mut hash = blockchain.lock().await.last_hash;

    while hash != [0; HEADER_PREVIOUS_BLOCK_HASH_SIZE] {
        let b = match blockchain.lock().await.get_block_from_hash(&hash) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Failed to get block: {}", e);
                res.status = Status::BadRequest;
                return res.send(b"Invalid block").await;
            }
        };

        for tr in b.transactions.iter().rev() {
            if tr.header.get_login() == *login {
                // take the transaction
            } else {
                match tr.get_data() {
                    Ok(tr_data) => match tr_data {
                        TransactionData::ClassicTransaction {
                            receiver,
                            ..
                        } => {
                            let mut l = [0; TRANSACTION_RECEIVER];
                            l[..login.len()].copy_from_slice(login.as_bytes());

                            if receiver != l {
                                continue; // skip this transaction
                            }

                            // take the transaction
                        }
                        _ => continue, // skip this transaction
                    },
                    _ => continue, // skip this transaction
                };
            }

            let obj = match serde_json::to_string(&tr) {
                Ok(obj) => obj,
                Err(e) => {
                    eprintln!("Failed to serialize transaction: {}", e);
                    res.status = Status::BadRequest;
                    return res.send(b"Failed to parse transaction").await;
                }
            };

            match arr.push(obj) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to add transaction object: {}", e);
                    res.status = Status::BadRequest;
                    return res.send(b"Failed to add transaction object").await;
                }
            }

            if arr.len() >= n {
                break;
            }
        }

        if arr.len() >= n {
            break;
        }

        hash = b.header.previous_block_hash;

        match blockchain.lock().await.hash_cache.get(&hash) {
            Some(0) => break, // end of blockchain
            Some(_) => {}     // continue
            None => {
                // block not found in cache
                eprintln!("Block not found in cache: {}", hex::encode(hash));
                res.status = Status::BadRequest;
                return res.send(b"Invalid block").await;
            }
        }
    }

    let data = arr.dump();
    let crypted = match key.crypt_split(&data) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to encrypt response: {}", e);
            res.status = Status::InternalServerError;
            return res.send(b"Failed to encrypt response").await;
        }
    };

    res.status = Status::Ok;
    res.set_header("content-type", "text/plain");
    res.send(crypted.as_bytes()).await
}
