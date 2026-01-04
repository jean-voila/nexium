use colored::Colorize;
use nexium::blockchain::transaction::Transaction;
use nexium::defaults::NEXIUM_HOME;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
    time::Duration,
};

const PEERS_FILE: &str = "peers.json";
const PEER_TIMEOUT_SECS: u64 = 5;
const BROADCAST_TIMEOUT_SECS: u64 = 2;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Peer {
    pub address: String,
    pub port: u16,
}

impl Peer {
    pub fn new(address: String, port: u16) -> Self {
        Self { address, port }
    }

    pub fn url(&self) -> String {
        format!("http://{}:{}", self.address, self.port)
    }

    /// Fetch the peer list from this peer (passive discovery, doesn't announce us)
    #[allow(dead_code)]
    pub async fn fetch_peers(&self) -> Result<Vec<Peer>, String> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(PEER_TIMEOUT_SECS))
            .build()
            .map_err(|e: reqwest::Error| e.to_string())?;

        let url = format!("{}/peers", self.url());
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e: reqwest::Error| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("Failed to fetch peers: {}", resp.status()));
        }

        let body = resp.text().await.map_err(|e: reqwest::Error| e.to_string())?;
        let peers: Vec<Peer> =
            serde_json::from_str(&body).map_err(|e| e.to_string())?;
        Ok(peers)
    }

    /// Announce ourselves to this peer and get their peer list in return
    pub async fn announce_self(&self, self_peer: &Peer) -> Result<Vec<Peer>, String> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(PEER_TIMEOUT_SECS))
            .build()
            .map_err(|e: reqwest::Error| e.to_string())?;

        let url = format!("{}/register_peer", self.url());
        let body = serde_json::to_string(self_peer).map_err(|e| e.to_string())?;
        
        let resp = client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .map_err(|e: reqwest::Error| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("Failed to register with peer: {}", resp.status()));
        }

        let body = resp.text().await.map_err(|e: reqwest::Error| e.to_string())?;
        let peers: Vec<Peer> =
            serde_json::from_str(&body).map_err(|e| e.to_string())?;
        Ok(peers)
    }

    /// Announce a third-party peer to this peer (for mesh propagation)
    pub async fn announce_peer(&self, peer_to_announce: &Peer) -> Result<(), String> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(BROADCAST_TIMEOUT_SECS))
            .build()
            .map_err(|e: reqwest::Error| e.to_string())?;

        let url = format!("{}/register_peer", self.url());
        let body = serde_json::to_string(peer_to_announce).map_err(|e| e.to_string())?;
        
        let resp = client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .map_err(|e: reqwest::Error| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("Failed to announce peer: {}", resp.status()));
        }

        Ok(())
    }

    /// Broadcast a transaction to this peer
    pub async fn broadcast_transaction(&self, transaction: &Transaction) -> Result<(), String> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(BROADCAST_TIMEOUT_SECS))
            .build()
            .map_err(|e: reqwest::Error| e.to_string())?;

        let url = format!("{}/sync_transaction", self.url());
        let body = serde_json::to_string(transaction).map_err(|e| e.to_string())?;
        
        let resp = client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .map_err(|e: reqwest::Error| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("Failed to sync transaction: {}", resp.status()));
        }

        Ok(())
    }

    /// Broadcast a block to this peer (sends raw block bytes as base64)
    pub async fn broadcast_block(&self, block_data: &[u8]) -> Result<(), String> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(BROADCAST_TIMEOUT_SECS))
            .build()
            .map_err(|e: reqwest::Error| e.to_string())?;

        let url = format!("{}/sync_block", self.url());
        use base64::{engine::general_purpose::STANDARD, Engine};
        let body = STANDARD.encode(block_data);
        
        let resp = client
            .post(&url)
            .header("Content-Type", "text/plain")
            .body(body)
            .send()
            .await
            .map_err(|e: reqwest::Error| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("Failed to sync block: {}", resp.status()));
        }

        Ok(())
    }

    /// Get blockchain info (size in blocks) from this peer
    pub async fn get_blockchain_info(&self) -> Result<BlockchainInfo, String> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(PEER_TIMEOUT_SECS))
            .build()
            .map_err(|e: reqwest::Error| e.to_string())?;

        let url = format!("{}/blockchain_info", self.url());
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e: reqwest::Error| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("Failed to get blockchain info: {}", resp.status()));
        }

        let body = resp.text().await.map_err(|e: reqwest::Error| e.to_string())?;
        let info: BlockchainInfo = serde_json::from_str(&body).map_err(|e| e.to_string())?;
        Ok(info)
    }

    /// Download the full blockchain from this peer
    pub async fn download_blockchain(&self) -> Result<Vec<u8>, String> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30)) // Longer timeout for full download
            .build()
            .map_err(|e: reqwest::Error| e.to_string())?;

        let url = format!("{}/blockchain_download", self.url());
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e: reqwest::Error| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("Failed to download blockchain: {}", resp.status()));
        }

        let body = resp.text().await.map_err(|e: reqwest::Error| e.to_string())?;
        use base64::{engine::general_purpose::STANDARD, Engine};
        STANDARD.decode(&body).map_err(|e| e.to_string())
    }
}

/// Blockchain info for synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainInfo {
    pub block_count: u64,
    pub size: u64,
    pub last_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerList {
    pub peers: Vec<Peer>,
}

impl PeerList {
    pub fn new() -> Self {
        Self { peers: vec![] }
    }

    /// Get the path to the peers file
    pub fn get_peers_file_path() -> String {
        let local_path = Path::new(NEXIUM_HOME);
        local_path.join(PEERS_FILE).to_string_lossy().to_string()
    }

    /// Generate a sample peers.json file with examples
    pub fn generate() -> Result<(), String> {
        let path = Self::get_peers_file_path();
        let file_path = Path::new(&path);

        // Create a sample peers file with example entries
        let sample_content = r#"{
  "_comment": "Nexium Peer List - Add known peers to bootstrap the network",
  "_example": "Each peer needs an 'address' (IP or hostname) and 'port' number",
  "peers": [
    {
      "_comment": "Example peer - replace with real peer addresses",
      "address": "192.168.1.100",
      "port": 8080
    },
    {
      "_comment": "You can also use hostnames",
      "address": "nexium.example.com",
      "port": 8080
    }
  ]
}
"#;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .map_err(|e| e.to_string())?;

        file.write_all(sample_content.as_bytes())
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Check if the peers file exists
    pub fn file_exists() -> bool {
        let path = Self::get_peers_file_path();
        Path::new(&path).exists()
    }

    /// Load the peer list from the file
    pub fn load() -> Self {
        let path = Self::get_peers_file_path();
        let file_path = Path::new(&path);

        if !file_path.exists() {
            return Self::new();
        }

        let mut file = match OpenOptions::new().read(true).open(file_path) {
            Ok(f) => f,
            Err(_) => return Self::new(),
        };

        let mut content = String::new();
        if file.read_to_string(&mut content).is_err() {
            return Self::new();
        }

        serde_json::from_str(&content).unwrap_or_else(|_| Self::new())
    }

    /// Save the peer list to the file
    pub fn save(&self) -> Result<(), String> {
        let path = Self::get_peers_file_path();
        let file_path = Path::new(&path);

        let content =
            serde_json::to_string_pretty(&self).map_err(|e| e.to_string())?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .map_err(|e| e.to_string())?;

        file.write_all(content.as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Add a peer to the list if not already present
    pub fn add_peer(&mut self, peer: Peer) -> bool {
        if !self.peers.iter().any(|p| p.address == peer.address && p.port == peer.port) {
            self.peers.push(peer);
            true
        } else {
            false
        }
    }

    /// Discover peers by contacting all known peers, announcing ourselves, and exchanging peer lists
    /// Also announces to newly discovered peers (mesh propagation)
    /// Returns (peers_added, best_peer_for_blockchain_sync)
    pub async fn discover(&mut self, self_address: &str, self_port: u16) -> (usize, Option<(Peer, BlockchainInfo)>) {
        let mut all_new_peers: HashSet<Peer> = HashSet::new();
        let self_peer = Peer::new(self_address.to_string(), self_port);
        let mut best_blockchain: Option<(Peer, BlockchainInfo)> = None;
        
        // Clone the current peers to iterate over
        let current_peers = self.peers.clone();
        
        for peer in current_peers {
            // Skip self
            if peer.address == self_address && peer.port == self_port {
                continue;
            }

            print!("Connecting to {} ... ", peer.url().yellow());
            
            // Announce ourselves to the peer and get their peer list
            match peer.announce_self(&self_peer).await {
                Ok(remote_peers) => {
                    println!(
                        "{} (received {} peers)",
                        "OK".green(),
                        remote_peers.len()
                    );
                    
                    // Get blockchain info from this peer
                    if let Ok(info) = peer.get_blockchain_info().await {
                        println!(
                            "  Blockchain: {} blocks, {} bytes",
                            info.block_count.to_string().cyan(),
                            info.size
                        );
                        
                        // Keep track of the peer with the longest chain
                        let dominated = match &best_blockchain {
                            Some((_, best_info)) => info.block_count > best_info.block_count,
                            None => info.block_count > 0,
                        };
                        if dominated {
                            best_blockchain = Some((peer.clone(), info));
                        }
                    }
                    
                    for remote_peer in remote_peers {
                        // Don't add ourselves
                        if remote_peer.address == self_address
                            && remote_peer.port == self_port
                        {
                            continue;
                        }
                        all_new_peers.insert(remote_peer);
                    }
                }
                Err(_) => {
                    println!("{}", "FAILED".red());
                }
            }
        }

        // Add all discovered peers and announce to newly discovered ones
        // Also check their blockchain info
        let mut added = 0;
        for peer in all_new_peers.clone() {
            if self.add_peer(peer.clone()) {
                print!(
                    "New peer discovered: {} ... ",
                    peer.url().green()
                );
                added += 1;
                
                // Announce ourselves to the newly discovered peer and check blockchain
                let self_peer_clone = self_peer.clone();
                match peer.announce_self(&self_peer_clone).await {
                    Ok(_) => {
                        print!("{}", "OK".green());
                        // Check blockchain info from this new peer
                        if let Ok(info) = peer.get_blockchain_info().await {
                            print!(
                                " ({} blocks)",
                                info.block_count.to_string().cyan()
                            );
                            // Update best blockchain if this one is better
                            let dominated = match &best_blockchain {
                                Some((_, best_info)) => info.block_count > best_info.block_count,
                                None => info.block_count > 0,
                            };
                            if dominated {
                                best_blockchain = Some((peer.clone(), info));
                            }
                        }
                        println!();
                    }
                    Err(_) => println!("{}", "FAILED".red()),
                }
            }
        }

        // Save the updated list
        if added > 0 {
            let _ = self.save();
        }

        (added, best_blockchain)
    }

    /// Broadcast a transaction to all peers (fire and forget, parallel)
    pub async fn broadcast_transaction(&self, transaction: &Transaction, self_address: &str, self_port: u16) {
        let peers = self.peers.clone();
        let transaction = transaction.clone();
        
        for peer in peers {
            // Skip self
            if peer.address == self_address && peer.port == self_port {
                continue;
            }
            
            let tr = transaction.clone();
            tokio::spawn(async move {
                let _ = peer.broadcast_transaction(&tr).await;
            });
        }
    }

    /// Broadcast a block to all peers (fire and forget, parallel)
    pub async fn broadcast_block(&self, block_data: Vec<u8>, self_address: &str, self_port: u16) {
        let peers = self.peers.clone();
        
        for peer in peers {
            // Skip self
            if peer.address == self_address && peer.port == self_port {
                continue;
            }
            
            let data = block_data.clone();
            tokio::spawn(async move {
                let _ = peer.broadcast_block(&data).await;
            });
        }
    }
}
