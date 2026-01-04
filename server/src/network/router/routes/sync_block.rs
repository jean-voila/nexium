use std::sync::Arc;
use tokio::sync::Mutex;
use colored::Colorize;
use base64::{engine::general_purpose::STANDARD, Engine};

use crate::{
    blockchain::{blockchain::Blockchain, structure::block::Block},
    network::router::http::{
        request::Request, response::Response, status::Status,
    },
};

/// Handler for receiving broadcasted blocks from peers
pub async fn handler(
    req: Request,
    blockchain: Arc<Mutex<Blockchain>>,
) {
    // Decode block from base64
    let block_data = match STANDARD.decode(&req.body) {
        Ok(d) => d,
        Err(_) => {
            let res = Response::new(Status::BadRequest, "Invalid block encoding");
            let _ = req.send(&res).await;
            return;
        }
    };

    // Parse the block
    let block = match Block::from_buffer(&block_data) {
        Ok(b) => b,
        Err(_) => {
            let res = Response::new(Status::BadRequest, "Invalid block format");
            let _ = req.send(&res).await;
            return;
        }
    };

    println!(
        "{} Block with {} transaction(s) (synced from peer)",
        "SYNC".cyan().bold(),
        block.transactions.len()
    );

    // Add block to blockchain
    let mut bc = blockchain.lock().await;
    
    // Verify the block connects to our chain
    if block.header.previous_block_hash != bc.last_hash {
        // Block doesn't connect - might be a fork or we're behind
        println!(
            "{} Block rejected: does not connect to chain (expected {:?}, got {:?})",
            "SYNC".red().bold(),
            hex::encode(&bc.last_hash[..8]),
            hex::encode(&block.header.previous_block_hash[..8])
        );
        let res = Response::new(Status::BadRequest, "Block does not connect to chain");
        let _ = req.send(&res).await;
        return;
    }

    bc.append_synced_block(&block);
    println!(
        "{} Block added to chain (now {} blocks)",
        "SYNC".green().bold(),
        bc.cache.len()
    );

    let res = Response::new(Status::Ok, "");
    let _ = req.send(&res).await;
}
