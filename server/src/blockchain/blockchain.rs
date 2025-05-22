use super::{
    mempool::Mempool,
    structure::{
        block::Block,
        block_header::{BlockHeader, HeaderPreviousBlockHash},
        consts::{BLOCK_HEADER_SIZE, HEADER_PREVIOUS_BLOCK_HASH_SIZE},
    },
};
use nexium::{
    blockchain::transaction::Transaction, defaults::BLOCKCHAIN_FILE,
    sha256::sha256,
};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::Write,
    os::unix::fs::FileExt,
};

pub struct Blockchain {
    pub cache: HashMap<HeaderPreviousBlockHash, u64>,
    file: File,
    pub last_hash: HeaderPreviousBlockHash,
    mempool: Mempool,
    size: u64,
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
            size: 0,
        };

        let blockchain_size = match b.file.metadata() {
            Ok(m) => m.len(),
            Err(_) => {
                return Err(format!(
                    "Failed to get blockchain file size: {}",
                    BLOCKCHAIN_FILE
                ));
            }
        };

        loop {
            if b.size >= blockchain_size {
                break;
            }

            let block = match b.read_block(b.size) {
                Ok(b) => b,
                Err(e) => {
                    return Err(format!(
                        "Failed to read blockchain file: {}",
                        e
                    ));
                }
            };

            match block.header.previous_block_hash {
                x if x == [0; HEADER_PREVIOUS_BLOCK_HASH_SIZE]
                    && b.last_hash == [0; HEADER_PREVIOUS_BLOCK_HASH_SIZE] => {}
                x if x == b.last_hash => {}
                _ => {
                    return Err("Invalid previous block hash".to_string());
                }
            }

            // dbg!(&block);
            b.last_hash = sha256(&block.to_buffer());
            // dbg!(b.last_hash);
            b.cache.insert(b.last_hash, b.size);
            b.size += BLOCK_HEADER_SIZE as u64
                + block.header.transactions_size as u64;
            dbg!(b.size, blockchain_size, block.size());
        }

        // dbg!(&b.cache);
        dbg!(&b.cache.len());
        Ok(b)
    }

    pub fn append(&mut self, block: &Block) {
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
            // dbg!(&block);
            self.append(&block);
        }
    }

    pub fn read_block(&mut self, offset: u64) -> Result<Block, String> {
        let mut header_buff = [0_u8; BLOCK_HEADER_SIZE];

        let header = match self.file.read_exact_at(&mut header_buff, offset) {
            Ok(_) => BlockHeader::from_buff(&header_buff),
            Err(e) => {
                return Err(format!("Error reading blockchain file: {}", e));
            }
        };

        let mut block_buff = vec![0_u8; header.transactions_size as usize];

        match self
            .file
            .read_exact_at(&mut block_buff, offset + BLOCK_HEADER_SIZE as u64)
        {
            Ok(_) => (),
            Err(e) => {
                return Err(format!("Failed to read blockchain file: {}", e));
            }
        };

        let mut buff =
            vec![0_u8; BLOCK_HEADER_SIZE + header.transactions_size as usize];
        buff[0..BLOCK_HEADER_SIZE].copy_from_slice(&header_buff);
        buff[BLOCK_HEADER_SIZE..].copy_from_slice(&block_buff);

        Block::from_buffer(&buff)
    }

    pub fn get_block(
        &mut self,
        hash: &HeaderPreviousBlockHash,
    ) -> Result<Block, String> {
        let offset = match self.cache.get(hash) {
            Some(o) => *o,
            None => {
                return Err("Block not found in cache".to_string());
            }
        };
        self.read_block(offset)
    }
}
