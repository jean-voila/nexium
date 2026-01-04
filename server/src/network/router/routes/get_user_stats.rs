use std::{ops::DerefMut, sync::Arc};

use crate::{
    blockchain::{
        blockchain::Blockchain, cache::cache::Cache,
        structure::consts::HEADER_PREVIOUS_BLOCK_HASH_SIZE,
    },
    network::router::http::{
        request::Request, response::Response, status::Status,
    },
};
use nexium::{
    blockchain::{consts::TRANSACTION_RECEIVER, transaction_data::TransactionData},
    utils::rand::create_noise,
};
use tokio::sync::Mutex;

pub async fn handler(
    req: Request,
    cache: Arc<Mutex<Cache>>,
    blockchain: Arc<Mutex<Blockchain>>,
) {
    let sp: Vec<String> = req.path.split("/").map(|e| e.to_string()).collect();
    let user_login = &sp[2];

    if user_login.is_empty() {
        let res = Response::new(Status::BadRequest, "");
        let _ = req.send(&res).await;
        return;
    }

    let key = match req.check(cache.lock().await.deref_mut()).await {
        Ok(data) => data,
        Err(e) => {
            let res = Response::new(Status::BadRequest, e);
            let _ = req.send(&res).await;
            return;
        }
    };

    // Get user balance
    let balance = match blockchain.lock().await.get_user_balance(user_login) {
        Ok(b) => b,
        Err(e) => {
            let res = Response::new(Status::BadRequest, e);
            let _ = req.send(&res).await;
            return;
        }
    };

    // Calculate stats by iterating through blockchain
    let mut sent_count: u64 = 0;
    let mut received_count: u64 = 0;
    let mut total_sent: f64 = 0.0;
    let mut total_received: f64 = 0.0;

    let mut hash = blockchain.lock().await.last_hash;

    while hash != [0; HEADER_PREVIOUS_BLOCK_HASH_SIZE] {
        let b = match blockchain.lock().await.get_block(&hash) {
            Ok(b) => b,
            Err(_) => break,
        };

        for tr in b.transactions.iter() {
            let emitter_login = tr.header.get_login();
            
            match tr.get_data() {
                Ok(TransactionData::ClassicTransaction {
                    receiver,
                    amount,
                    ..
                }) => {
                    let mut login_bytes = [0; TRANSACTION_RECEIVER];
                    login_bytes[..user_login.len()]
                        .copy_from_slice(user_login.as_bytes());

                    // Check if user sent this transaction
                    if emitter_login == *user_login {
                        sent_count += 1;
                        total_sent += amount as f64;
                    }

                    // Check if user received this transaction
                    if receiver == login_bytes {
                        received_count += 1;
                        total_received += amount as f64;
                    }
                }
                _ => continue,
            }
        }

        hash = b.header.previous_block_hash;

        match blockchain.lock().await.cache.get(&hash) {
            Some(0) => break,
            Some(_) => {}
            None => break,
        }
    }

    let json = json::object! {
        "balance" => balance,
        "sent_count" => sent_count,
        "received_count" => received_count,
        "total_sent" => total_sent,
        "total_received" => total_received,
        "total_transactions" => sent_count + received_count,
        "noise" => create_noise(),
    };

    let data = json.dump();

    let crypted = match key.crypt_split(&data) {
        Ok(res) => res,
        Err(_) => {
            let res = Response::new(Status::InternalError, "");
            let _ = req.send(&res).await;
            return;
        }
    };

    let mut res = Response::new(Status::Ok, crypted);
    res.set_header("content-type", "text/plain");
    let _ = req.send(&res).await;
}
