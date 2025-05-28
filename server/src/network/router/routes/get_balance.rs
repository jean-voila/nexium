use std::{ops::DerefMut, sync::Arc};

use crate::{
    blockchain::{blockchain::Blockchain, cache::cache::Cache},
    network::router::http::{
        request::Request, response::Response, status::Status,
    },
};
use nexium::utils::rand::create_noise;
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
    // println!("login: {login}");

    let key = match req.check(cache.lock().await.deref_mut()).await {
        Ok(data) => data,
        Err(e) => {
            let res = Response::new(Status::BadRequest, e);
            let _ = req.send(&res).await;
            return;
        }
    };

    let balance = match blockchain.lock().await.get_user_balance(user_login) {
        Ok(b) => b,
        Err(e) => {
            let res = Response::new(Status::BadRequest, e);
            let _ = req.send(&res).await;
            return;
        }
    };

    let json = json::object! {
        "balance"=> balance,
        "noise"=> create_noise(),
    };

    let data = json.dump();
    println!("data: {}", data);
    println!("data len: {}", data.len());

    let crypted = match key.crypt(&data) {
        Ok(res) => res,
        Err(_) => {
            let res = Response::new(Status::InternalError, "");
            let _ = req.send(&res).await;
            return;
        }
    };
    println!("crypted: {}", crypted);

    let mut res = Response::new(Status::Ok, crypted);
    res.set_header("content-type", "text/plain");
    // let mut res = Response::new(Status::Ok, json.dump());
    // res.set_header("content-type", "text/json");
    let _ = req.send(&res).await;
}
