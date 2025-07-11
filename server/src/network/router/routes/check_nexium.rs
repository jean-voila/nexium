use nexium::{defaults::SIG_SAMPLE, gitlab::GitlabClient, rsa::KeyPair};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::network::http::{
    request::Request, response::Response, status::Status,
};

pub async fn handler(
    req: Request,
    mut res: Response,
    gitlab: Arc<Mutex<GitlabClient>>,
    login: Arc<String>,
    server_key: Arc<KeyPair>,
) -> Result<(), std::io::Error> {
    let sig = match server_key.sign(SIG_SAMPLE) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to sign sample: {}", e);
            res.status = Status::InternalServerError;
            return res.send(b"Failed to sign sample").await;
        }
    };

    let json = json::object! {
        login: login.to_string(),
        sigSample: sig.to_string(),
        // version: 0,
    };

    let key = match req.get_key(&gitlab).await {
        Ok(data) => data,
        Err(e) => {
            res.status = Status::Unauthorized;
            return res.send(e.as_bytes()).await;
        }
    };

    let data = json.dump();
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
