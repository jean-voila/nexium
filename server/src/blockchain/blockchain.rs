use super::{
    mempool::Mempool,
    structure::{block::Block, block_header::HeaderPreviousBlockHash},
};
use nexium::{
    blockchain::transaction::Transaction, defaults::BLOCKCHAIN_FILE,
    sha256::sha256,
};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

pub struct Blockchain {
    cache: HashMap<String, HeaderPreviousBlockHash>,
    file: File,
    last_hash: HeaderPreviousBlockHash,
    mempool: Mempool,
}

impl Blockchain {
    pub fn init() -> Result<Self, String> {
        let r = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(BLOCKCHAIN_FILE);

        let mut file = match r {
            Ok(f) => f,
            Err(_) => {
                return Err(format!(
                    "Failed to open blockchain file: {}",
                    BLOCKCHAIN_FILE
                ));
            }
        };

        // read the file and load the blocks into the cache

        let mut b = Self {
            cache: HashMap::new(),
            file,
            last_hash: HeaderPreviousBlockHash::default(),
            mempool: Mempool::new(),
        };

        // let mut buff = vec![];
        // let mut r = 1;
        // while r != 0 {
        //     buff.clear();
        //     r = file.read(&mut buff).unwrap_or(0);
        // }

        // b.cache.insert(k, v)
        Ok(b)
    }

    fn append(&mut self, block: &Block) {
        let buff = block.to_buffer();
        match self.file.write_all(&buff) {
            Ok(_) => {
                self.last_hash = sha256(&buff);
                // self.cache.insert();
            }
            Err(e) => {
                println!("Error writing to file: {}", e);
            }
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.mempool.add(transaction);
        dbg!(self.mempool.is_full());
        if self.mempool.is_full() {
            let block = Block::new(self.last_hash, self.mempool.dump());
            dbg!(&block);
            self.append(&block);
        }
    }
}
