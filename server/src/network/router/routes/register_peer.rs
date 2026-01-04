use std::sync::Arc;
use tokio::sync::Mutex;
use colored::Colorize;

use crate::{
    network::router::http::{
        request::Request, response::Response, status::Status,
    },
    peers::{Peer, PeerList},
};

pub async fn handler(
    req: Request, 
    peer_list: Arc<Mutex<PeerList>>, 
    _self_login: String,
    self_address: String,
    self_port: u16,
) {
    // Parse the incoming peer registration
    let new_peer: Peer = match serde_json::from_str(&req.body) {
        Ok(p) => p,
        Err(_) => {
            let res = Response::new(Status::BadRequest, "Invalid peer format");
            let _ = req.send(&res).await;
            return;
        }
    };

    let mut peers = peer_list.lock().await;
    
    let peer_url = new_peer.url();
    let is_new = peers.add_peer(new_peer.clone());
    
    if is_new {
        println!(
            "{} New peer discovered us: {}",
            "MESH".cyan().bold(),
            peer_url.yellow()
        );
        let _ = peers.save();
        
        // Propagate the new peer to all our other peers
        let other_peers: Vec<Peer> = peers.peers.iter()
            .filter(|p| {
                // Skip the new peer itself and ourselves
                !(p.address == new_peer.address && p.port == new_peer.port) &&
                !(p.address == self_address && p.port == self_port)
            })
            .cloned()
            .collect();
        
        // Announce the new peer to all our other peers in the background
        for other_peer in other_peers {
            let new_peer_clone = new_peer.clone();
            tokio::spawn(async move {
                let _ = other_peer.announce_peer(&new_peer_clone).await;
            });
        }
    }

    // Return our peer list
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
