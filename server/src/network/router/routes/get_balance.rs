use std::{ops::DerefMut, sync::Arc};

use crate::{
    blockchain::blockchain::Blockchain,
    network::http::{request::Request, response::Response, status::Status},
};
use nexium::{gitlab::GitlabClient, utils::rand::create_noise};
use tokio::sync::Mutex;

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

    let balance = match blockchain.lock().await.get_balance(login) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to get user balance: {}", e);
            res.status = Status::BadRequest;
            return res.send(b"Failed to get user balance").await;
        }
    };

    let json = json::object! {
        "balance"=> balance,
        "noise"=> create_noise(),
    };

    let data = json.dump();
    let crypted = match key.crypt(&data) {
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
