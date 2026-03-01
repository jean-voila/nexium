use crate::nexium_api::PeerInfo;

pub fn get_peers_cache_path() -> std::path::PathBuf {
    let mut path =
        dirs::data_local_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    path.push("nexium");
    let _ = std::fs::create_dir_all(&path);
    path.push("peers_cache.json");
    path
}

/// Get cached peers from local storage
pub fn get_cached_peers() -> Vec<PeerInfo> {
    let path = get_peers_cache_path();
    if !path.exists() {
        return vec![];
    }

    match std::fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => vec![],
    }
}

/// Save peers to local cache
pub fn save_peers_cache(peers: &[PeerInfo]) {
    let path = get_peers_cache_path();
    if let Ok(content) = serde_json::to_string(peers) {
        let _ = std::fs::write(&path, content);
    }
}
