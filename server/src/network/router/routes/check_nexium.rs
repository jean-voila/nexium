use nexium::{defaults::SIG_SAMPLE, rsa::KeyPair};
use std::{ops::DerefMut, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    blockchain::cache::cache::Cache,
    network::router::http::{
        request::Request, response::Response, status::Status,
    },
};

pub async fn handler(
    req: Request,
    cache: Arc<Mutex<Cache>>,
    login: String,
    key: KeyPair,
) {
    let sig = match key.sign(SIG_SAMPLE) {
        Ok(s) => s,
        Err(_) => {
            let res = Response::new(Status::InternalError, "");
            let _ = req.send(&res).await;
            return;
        }
    };

    let json = json::object! {
        login: login.clone(),
        sigSample: sig.to_string(),
        // version: 0,
    };

    let key = match req.check(cache.lock().await.deref_mut()).await {
        Ok(data) => data,
        Err(e) => {
            let res = Response::new(Status::BadRequest, e);
            let _ = req.send(&res).await;
            return;
        }
    };

    println!("User connected: {}", login);

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
