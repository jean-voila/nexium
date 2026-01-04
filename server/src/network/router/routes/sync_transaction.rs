use std::sync::Arc;
use tokio::sync::Mutex;
use colored::Colorize;

use nexium::blockchain::transaction::Transaction;

use crate::{
    blockchain::blockchain::Blockchain,
    network::router::http::{
        request::Request, response::Response, status::Status,
    },
    peers::PeerList,
};

/// Handler for receiving broadcasted transactions from peers
pub async fn handler(
    req: Request,
    blockchain: Arc<Mutex<Blockchain>>,
    _peer_list: Arc<Mutex<PeerList>>,
    _self_address: String,
    _self_port: u16,
) {
    // Parse the transaction from JSON
    let transaction: Transaction = match serde_json::from_str(&req.body) {
        Ok(t) => t,
        Err(_) => {
            let res = Response::new(Status::BadRequest, "Invalid transaction format");
            let _ = req.send(&res).await;
            return;
        }
    };

    let emitter = transaction.header.get_login();
    println!(
        "{} Transaction from {} (synced from peer)",
        "SYNC".cyan().bold(),
        emitter.yellow()
    );

    // Add to blockchain (this will NOT re-broadcast since it's already synced)
    blockchain
        .lock()
        .await
        .add_transaction_from_sync(transaction)
        .await;

    let res = Response::new(Status::Ok, "");
    let _ = req.send(&res).await;
}
