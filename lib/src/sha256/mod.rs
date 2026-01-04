use sha2::{Digest, Sha256};

/// Compute SHA-256 hash of the input bytes
pub fn sha256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().into()
}
