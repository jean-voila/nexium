use std::sync::Arc;

use nexium::{
    blockchain::{
        consts::TRANSACTION_RECEIVER, transaction_data::TransactionData,
    },
    gitlab::GitlabClient,
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
) -> Result<(), std::io::Error> {
    let sp: Vec<String> = req.path.split("/").map(|e| e.to_string()).collect();
    let login = &sp[2];

    if login.is_empty() {
        res.status = Status::BadRequest;
        return res.send(b"Missing user login").await;
    }
    // println!("login: {}", login);

    let n = match req.query.get("n") {
        Some(n) => match n.parse::<usize>() {
            Ok(0) | Err(_) => 3,
            Ok(100..) => 100,
            Ok(x) => x,
        },
        None => 3,
    };
    // println!("n: {n}");

    let user_exists =
        gitlab.lock().await.check_user_existence_async(login).await;

    match user_exists {
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

    let key = match req.get_key(&gitlab).await {
        Ok(data) => data,
        Err(e) => {
            res.status = Status::Unauthorized;
            return res.send(e.as_bytes()).await;
        }
    };

    // if lock is slowing down the server, consider using a read lock or cloning the blockchain struct
    let blockchain_lock = blockchain.lock().await;
    let iter = match blockchain_lock.iter_rev() {
        Ok(iter) => iter,
        Err(e) => {
            eprintln!("Failed to get blockchain iterator: {}", e);
            res.status = Status::InternalServerError;
            return res.send(b"Failed to get blockchain iterator").await;
        }
    };

    let mut arr = json::array![];
    let mut log_bytes = [0; TRANSACTION_RECEIVER];
    log_bytes[..login.len()].copy_from_slice(login.as_bytes());

    for block in iter {
        let b = match block {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Failed to get block: {}", e);
                res.status = Status::BadRequest;
                return res.send(b"Invalid block").await;
            }
        };

        for tr in b.transactions.iter().rev() {
            // if header login is the same as the requested login -> take the transaction

            // if the login is not the same as the transaction login, check if the transaction receiver is the same as the requested login
            if tr.header.get_login() != *login {
                match tr.get_data() {
                    Ok(TransactionData::ClassicTransaction {
                        receiver,
                        ..
                    }) => {
                        if receiver != log_bytes {
                            continue; // skip this transaction
                        }

                        // take the transaction
                    }
                    _ => continue, // skip this transaction
                }
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
    }
    drop(blockchain_lock); // Explicitly drop the lock to avoid deadlock

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
