use std::sync::Arc;
use tokio::sync::Mutex;
use base64::{engine::general_purpose::STANDARD, Engine};

use crate::{
    blockchain::blockchain::Blockchain,
    network::router::http::{
        request::Request, response::Response, status::Status,
    },
};

/// Handler for downloading the full blockchain
pub async fn handler(req: Request, blockchain: Arc<Mutex<Blockchain>>) {
    let mut bc = blockchain.lock().await;
    
    let data = match bc.read_all() {
        Ok(d) => d,
        Err(_) => {
            let res = Response::new(Status::InternalError, "Failed to read blockchain");
            let _ = req.send(&res).await;
            return;
        }
    };
    
    drop(bc);

    let encoded = STANDARD.encode(&data);
    let res = Response::new(Status::Ok, &encoded);
    let _ = req.send(&res).await;
}
