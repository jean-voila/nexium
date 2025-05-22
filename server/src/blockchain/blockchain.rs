use super::structure::{block::Block, block_header::HeaderPreviousBlockHash};
use std::{collections::HashMap, fs::File};

const BLOCKCHAIN_FILE: &str = "blockchain.dat";

pub struct Blockchain {
    cache: HashMap<String, HeaderPreviousBlockHash>,
    file: File,
    last_hash: HeaderPreviousBlockHash,
}

impl Blockchain {
    pub fn init() -> Result<Self, String> {
        let file = match File::open(BLOCKCHAIN_FILE) {
            Ok(f) => f,
            Err(_) => match File::create(BLOCKCHAIN_FILE) {
                Ok(f) => f,
                Err(e) => {
                    return Err(format!("{} -> {}", BLOCKCHAIN_FILE, e));
                }
            },
        };

        // read the file and load the blocks into the cache

        let mut b = Self {
            cache: HashMap::new(),
            file,
            last_hash: HeaderPreviousBlockHash::default(),
        };
        // b.cache.insert(k, v)
        Ok(b)
    }

    pub fn append(&mut self, block: Block) {
        todo!()
    }
}
