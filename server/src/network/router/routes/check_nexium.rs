use nexium::{defaults::SIG_SAMPLE, rsa::KeyPair};
use std::{ops::DerefMut, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    blockchain::cache::cache::Cache,
    network::http::{request::Request, response::Response, status::Status},
};

pub async fn handler(
    req: Request,
    mut res: Response,
    cache: Arc<Mutex<Cache>>,
    login: String,
    server_key: KeyPair,
) {
    let sig = match server_key.sign(SIG_SAMPLE) {
        Ok(s) => s,
        Err(e) => {
            res.status = Status::InternalServerError;
            res.send(b"Failed to sign sample").await;
            return;
        }
    };

    let json = json::object! {
        login: login,
        sigSample: sig.to_string(),
        // version: 0,
    };

    let key = match req.check(cache.lock().await.deref_mut()).await {
        Ok(data) => data,
        Err(e) => {
            // let res = Response::new(, e);
            res.status = Status::BadRequest;
            res.send(b"Invalid request").await;
            return;
        }
    };

    let data = json.dump();
    let crypted = match key.crypt_split(&data) {
        Ok(res) => res,
        Err(e) => {
            res.status = Status::InternalServerError;
            res.send(b"Failed to encrypt response").await;
            return;
        }
    };

    // let mut res = Response::new(, crypted);
    res.status = Status::Ok;
    res.set_header("content-type", "text/plain");
    res.send(crypted).await;
}
