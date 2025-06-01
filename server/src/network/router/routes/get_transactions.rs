use std::{ops::DerefMut, sync::Arc};

use nexium::blockchain::{
    consts::TRANSACTION_RECEIVER, transaction_data::TransactionData,
};
use tokio::sync::Mutex;

use crate::{
    blockchain::{
        blockchain::Blockchain, cache::cache::Cache,
        structure::consts::HEADER_PREVIOUS_BLOCK_HASH_SIZE,
    },
    network::http::{request::Request, response::Response, status::Status},
};

pub async fn handler(
    req: Request,
    mut res: Response,
    cache: Arc<Mutex<Cache>>,
    blockchain: Arc<Mutex<Blockchain>>,
) {
    let sp: Vec<String> = req.path.split("/").map(|e| e.to_string()).collect();
    let login = &sp[2];

    if login.is_empty() {
        // let res = Response::new(Status::BadRequest, "");
        res.status = Status::BadRequest;
        res.send(b"Missing user login").await;
        return;
    }
    println!("login: {login}");

    let n = match req.query.get("n") {
        Some(n) => match n.parse::<usize>() {
            Ok(0) | Err(_) => 3,
            Ok(100..) => 100,
            Ok(x) => x,
        },
        None => 3,
    };
    println!("n: {n}");

    let key = match req.check(cache.lock().await.deref_mut()).await {
        Ok(data) => data,
        Err(e) => {
            // let res = Response::new(Status::BadRequest, e);
            res.status = Status::BadRequest;
            res.send(b"Invalid request").await;
            return;
        }
    };

    let mut arr = json::array![];
    let mut hash = blockchain.lock().await.last_hash;

    while hash != [0; HEADER_PREVIOUS_BLOCK_HASH_SIZE] {
        let b = match blockchain.lock().await.get_block(&hash) {
            Ok(b) => b,
            Err(_) => {
                res.status = Status::BadRequest;
                res.send(b"Invalid block").await;
                return;
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
                Err(_) => {
                    res.status = Status::BadRequest;

                    res.send(b"Failed to parse transaction").await;
                    return;
                }
            };

            match arr.push(obj) {
                Ok(_) => {}
                Err(_) => {
                    res.status = Status::BadRequest;
                    res.send(b"Failed to add transaction object").await;
                    return;
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

        match blockchain.lock().await.cache.get(&hash) {
            Some(0) => {
                // end of blockchain
                break;
            }
            Some(_) => {} // continue
            None => {
                // block not found in cache
                res.status = Status::BadRequest;
                res.send(b"Invalid block").await;
                return;
            }
        }
    }

    let data = arr.dump();
    let crypted = match key.crypt_split(&data) {
        Ok(res) => res,
        Err(_) => {
            res.status = Status::InternalServerError;
            res.send(b"Failed to encrypt response").await;
            return;
        }
    };

    res.status = Status::Ok;
    res.set_header("content-type", "text/plain");
    res.send(crypted).await;
}
