use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    network::router::http::{
        request::Request, response::Response, status::Status,
    },
    peers::PeerList,
};

pub async fn handler(req: Request, peer_list: Arc<Mutex<PeerList>>) {
    let peers = peer_list.lock().await;
    
    let json = match serde_json::to_string(&peers.peers) {
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
