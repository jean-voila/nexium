use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    blockchain::blockchain::Blockchain,
    network::router::http::{
        request::Request, response::Response, status::Status,
    },
};

/// Handler for getting blockchain info (for sync decisions)
pub async fn handler(req: Request, blockchain: Arc<Mutex<Blockchain>>) {
    let bc = blockchain.lock().await;
    let info = bc.get_info();
    drop(bc);

    let json = match serde_json::to_string(&info) {
        Ok(j) => j,
        Err(_) => {
            let res = Response::new(Status::InternalError, "");
            let _ = req.send(&res).await;
            return;
        }
    };

    let res = Response::new(Status::Ok, &json);
    let _ = req.send(&res).await;
}
