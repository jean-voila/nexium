use std::{ops::DerefMut, sync::Arc};

use crate::{
    blockchain::{blockchain::Blockchain, cache::cache::Cache},
    network::http::{request::Request, response::Response, status::Status},
};
use nexium::utils::rand::create_noise;
use tokio::sync::Mutex;

pub async fn handler(
    req: Request,
    mut res: Response,
    cache: Arc<Mutex<Cache>>,
    blockchain: Arc<Mutex<Blockchain>>,
) {
    let sp: Vec<String> = req.path.split("/").map(|e| e.to_string()).collect();
    let user_login = &sp[2];

    if user_login.is_empty() {
        res.status = Status::BadRequest;
        res.send(b"Missing user login").await;
        return;
    }
    // println!("login: {login}");

    let key = match req.check(cache.lock().await.deref_mut()).await {
        Ok(data) => data,
        Err(e) => {
            // let res = Response::new(Status::BadRequest, e);
            res.status = Status::BadRequest;
            res.send(b"Invalid request").await;
            return;
        }
    };

    let balance = match blockchain.lock().await.get_user_balance(user_login) {
        Ok(b) => b,
        Err(e) => {
            res.status = Status::BadRequest;
            res.send(b"Failed to get user balance").await;
            return;
        }
    };

    let json = json::object! {
        "balance"=> balance,
        "noise"=> create_noise(),
    };

    let data = json.dump();
    let crypted = match key.crypt(&data) {
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
