use super::{
    mempool::Mempool,
    structure::{
        block::Block,
        block_header::{BlockHeader, HeaderPreviousBlockHash},
        consts::BLOCK_HEADER_SIZE,
    },
};
use crate::peers::PeerList;
use nexium::{
    blockchain::{
        consts::TRANSACTION_RECEIVER, data_type::DataType,
        transaction::Transaction, transaction_data::TransactionData,
    },
    defaults::{BLOCKCHAIN_FILE, INITIAL_BALANCE},
    gitlab::GitlabClient,
};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    sync::Arc,
};
use tokio::sync::Mutex;

pub struct Blockchain {
    pub cache: HashMap<HeaderPreviousBlockHash, u64>,
    file: File,
    pub last_hash: HeaderPreviousBlockHash,
    mempool: Mempool,
    pub size: u64,
    gitlab: GitlabClient,
}

impl Blockchain {
    // fn create_genesis() -> Block {
    //     let t = Transaction::new(
    //         "GENESIS".as_bytes().to_vec(),
    //         0,
    //         "",
    //         DataType::Unknown,
    //         &key,
    //     );
    // }

    pub fn init(gitlab: GitlabClient) -> Result<Self, String> {
        let r = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(BLOCKCHAIN_FILE);

        let file = match r {
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
            gitlab,
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

        if blockchain_size == 0 {
            // Empty blockchain
        }

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
                x if x == b.last_hash => {}
                _ => {
                    return Err("Invalid previous block hash".to_string());
                }
            }

            // dbg!(&block);
            b.last_hash = block.double_hash();
            // dbg!(b.last_hash);
            b.cache.insert(b.last_hash, b.size);
            b.size += BLOCK_HEADER_SIZE as u64
                + block.header.transactions_size as u64;
            // dbg!(b.size, blockchain_size, block.size());
        }

        // dbg!(&b.cache);
        // dbg!(&b.cache.len());
        Ok(b)
    }

    pub fn append(&mut self, block: &Block) {
        let buff = block.to_buffer();
        match self.file.write_all(&buff) {
            Ok(_) => {
                // Flush to disk
                let _ = self.file.sync_all();
                self.last_hash = Block::double_hash_(&buff);
                self.cache.insert(self.last_hash, self.size);
                self.size += buff.len() as u64;
            }
            Err(e) => {
                eprintln!("Failed to write block to file: {}", e);
            }
        }
    }

    /// Append a block received from peer sync, also clears matching transactions from mempool
    pub fn append_synced_block(&mut self, block: &Block) {
        // Remove transactions that are in the block from our mempool
        self.mempool.remove_transactions(&block.transactions);
        // Append the block
        self.append(block);
    }

    /// Get blockchain info for synchronization
    pub fn get_info(&self) -> crate::peers::BlockchainInfo {
        crate::peers::BlockchainInfo {
            block_count: self.cache.len() as u64,
            size: self.size,
            last_hash: hex::encode(&self.last_hash),
        }
    }

    /// Read the entire blockchain file as bytes
    pub fn read_all(&mut self) -> Result<Vec<u8>, String> {
        self.file.seek(SeekFrom::Start(0)).map_err(|e| e.to_string())?;
        let mut data = vec![0u8; self.size as usize];
        self.file.read_exact(&mut data).map_err(|e| e.to_string())?;
        Ok(data)
    }

    /// Replace the entire blockchain with downloaded data
    pub fn replace_from_data(&mut self, data: &[u8]) -> Result<(), String> {
        // Open file with truncate to overwrite
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(BLOCKCHAIN_FILE)
            .map_err(|e| e.to_string())?;
        
        file.write_all(data).map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;
        
        // Reopen for append
        self.file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(BLOCKCHAIN_FILE)
            .map_err(|e| e.to_string())?;
        
        // Rebuild cache
        self.cache.clear();
        self.last_hash = HeaderPreviousBlockHash::default();
        self.size = 0;
        
        let blockchain_size = data.len() as u64;
        
        loop {
            if self.size >= blockchain_size {
                break;
            }

            let block = self.read_block(self.size)?;

            if block.header.previous_block_hash != self.last_hash {
                return Err("Invalid blockchain data: hash mismatch".to_string());
            }

            self.last_hash = block.double_hash();
            self.cache.insert(self.last_hash, self.size);
            self.size += BLOCK_HEADER_SIZE as u64 + block.header.transactions_size as u64;
        }
        
        Ok(())
    }

    async fn create_new_block(&mut self, peer_list: Arc<Mutex<PeerList>>, self_address: String, self_port: u16) {
        let mut transactions = self.mempool.dump();
        transactions
            .sort_by(|a, b| a.header.timestamp.cmp(&b.header.timestamp));

        let mut balances: HashMap<String, f32> = HashMap::new();
        let mut valid_trs: Vec<Transaction> = vec![];

        for tr in transactions.iter() {
            match tr.get_data() {
                Ok(data) => match data {
                    TransactionData::ClassicTransaction {
                        receiver,
                        amount,
                        ..
                    } => {
                        if amount <= 0 as f32 {
                            continue; // Invalid transaction amount
                        }

                        let em = tr.header.get_login();
                        let r = String::from_utf8_lossy(&receiver)
                            .trim_end_matches('\0')
                            .to_string();

                        if em == r {
                            continue; // Cannot send money to yourself
                        }

                        match self.gitlab.check_user_existence_async(&r).await {
                            Ok(exists) => {
                                if !exists {
                                    continue; // Receiver does not exist
                                }
                            }
                            Err(_) => {
                                continue; // Error checking user existence
                            }
                        }

                        let mut be = match balances.get(&em) {
                            Some(b) => *b,
                            None => match self.get_user_balance(&em) {
                                Ok(b) => b,
                                Err(_) => {
                                    continue; // Error getting emitter balance
                                }
                            },
                        };

                        let mut br = match balances.get(&r) {
                            Some(b) => *b,
                            None => match self.get_user_balance(&r) {
                                Ok(b) => b,
                                Err(_) => {
                                    continue; // Error getting receiver balance
                                }
                            },
                        };

                        // Calculate total cost: amount + transaction fees
                        let fee_cost = tr.fee_cost();
                        let total_cost = amount + fee_cost;

                        if (be as i64 - total_cost as i64) < 0 {
                            continue; // Insufficient balance (including fees)
                        } else {
                            be -= total_cost; // Deduct amount + fees from sender
                            br += amount; // Only the amount goes to receiver (fees are "burned")
                            balances.insert(em, be);
                            balances.insert(r.to_string(), br);
                            valid_trs.push(tr.clone()); // Valid transaction
                        }
                    }
                    _ => valid_trs.push(tr.clone()), // Other transaction types are considered valid
                },
                Err(_) => continue, // Error getting transaction data
            }
        }

        if valid_trs.is_empty() {
            return;
        }

        let block = Block::new(self.last_hash, &valid_trs);
        let block_data = block.to_buffer();
        self.append(&block);
        println!("New block created with {} transaction(s)", valid_trs.len());

        // Broadcast block to all peers
        let peers = peer_list.lock().await;
        peers.broadcast_block(block_data, &self_address, self_port).await;
    }

    /// Add a transaction from a client (will be broadcasted to peers)
    pub async fn add_transaction(&mut self, transaction: Transaction, peer_list: Arc<Mutex<PeerList>>, self_address: String, self_port: u16) {
        let emitter = transaction.header.get_login();
        let fees = transaction.fee_cost();
        
        match transaction.get_data() {
            Ok(TransactionData::ClassicTransaction { receiver, amount, .. }) => {
                let receiver_str = String::from_utf8_lossy(&receiver)
                    .trim_end_matches('\0')
                    .to_string();
                println!(
                    "Transaction: {} -> {} | amount: {} | fees: {}",
                    emitter, receiver_str, amount, fees
                );
            }
            _ => {
                println!("Transaction received from {}", emitter);
            }
        }
        
        self.mempool.add(transaction);

        if self.mempool.is_full() {
            self.create_new_block(peer_list, self_address, self_port).await;
        }
    }

    /// Add a transaction received from peer sync (no broadcast, no duplicate)
    pub async fn add_transaction_from_sync(&mut self, transaction: Transaction) {
        let _emitter = transaction.header.get_login();
        
        // Check if transaction is already in mempool (avoid duplicates)
        // For now, just add it - deduplication could be improved
        self.mempool.add(transaction);

        // Note: We don't create blocks from synced transactions
        // Only the originating server creates the block and broadcasts it
    }

    pub fn read_block(&mut self, offset: u64) -> Result<Block, String> {
        let mut header_buff = [0_u8; BLOCK_HEADER_SIZE];

        match self.file.seek(SeekFrom::Start(offset)) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!(
                    "Failed to seek in blockchain file: {}",
                    e
                ));
            }
        };

        let header = match self.file.read_exact(&mut header_buff) {
            Ok(_) => BlockHeader::from_buff(&header_buff),
            Err(e) => {
                return Err(format!("Error reading blockchain file: {}", e));
            }
        };

        let mut block_buff = vec![0_u8; header.transactions_size as usize];

        match self
            .file
            .seek(SeekFrom::Start(offset + BLOCK_HEADER_SIZE as u64))
        {
            Ok(_) => (),
            Err(e) => {
                return Err(format!(
                    "Failed to seek in blockchain file: {}",
                    e
                ));
            }
        };

        match self.file.read_exact(&mut block_buff) {
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

    pub fn block_foreach(
        &mut self,
        mut f: impl FnMut(&Block) -> Result<(), String>,
    ) -> Result<(), String> {
        let mut offset = 0;
        while offset < self.size {
            let block = match self.read_block(offset) {
                Ok(b) => b,
                Err(e) => {
                    return Err(format!(
                        "Error reading blockchain file: {}",
                        e
                    ));
                }
            };
            f(&block)?;
            offset += block.size() as u64;
        }
        Ok(())
    }

    pub fn get_user_balance<T>(&mut self, login: T) -> Result<f32, String>
    where
        T: AsRef<str>,
    {
        let login = login.as_ref();
        let mut balance = INITIAL_BALANCE as f32;

        let res = self.block_foreach(|b| {
            for tr in &b.transactions {
                if tr.header.data_type == DataType::ClassicTransaction {
                    match tr.get_data() {
                        Ok(data) => {
                            match data {
                                TransactionData::ClassicTransaction {
                                    receiver,
                                    amount,
                                    ..
                                } => {
                                    let mut l = [0; TRANSACTION_RECEIVER];
                                    l[..login.len()]
                                        .copy_from_slice(login.as_bytes());
                                    if l == receiver {
                                        balance += amount;
                                    } else if l == tr.header.emitter {
                                        // Deduct the amount + transaction fees
                                        balance -= amount + tr.fee_cost();
                                    };
                                }
                                _ => (),
                            };
                        }
                        Err(_) => {
                            return Err(
                                "Failed to get transaction data".to_string()
                            );
                        }
                    };
                };
            }
            Ok(())
        });

        match res {
            Ok(_) => Ok(balance),
            Err(e) => Err(e),
        }
    }
}
