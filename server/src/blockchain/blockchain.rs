use super::{
    mempool::Mempool,
    structure::{
        block::Block,
        block_header::{BlockHeader, HeaderPreviousBlockHash},
        consts::BLOCK_HEADER_SIZE,
    },
};
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
};

pub struct Blockchain<'a> {
    pub cache: HashMap<HeaderPreviousBlockHash, u64>,
    file: File,
    pub last_hash: HeaderPreviousBlockHash,
    mempool: Mempool,
    pub size: u64,
    gitlab: &'a GitlabClient,
}

impl<'a> Blockchain<'a> {
    pub fn init(gitlab: &'a GitlabClient) -> Result<Self, String> {
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
                self.last_hash = Block::double_hash_(&buff);
                self.cache.insert(self.last_hash, self.size);
                self.size += buff.len() as u64;
            }
            Err(e) => {
                println!("Error writing to file: {}", e);
            }
        }
    }

    fn create_new_block(&mut self) {
        let mut transactions = self.mempool.dump();
        transactions
            .sort_by(|a, b| a.header.timestamp.cmp(&b.header.timestamp));

        let mut balances: HashMap<String, f32> = HashMap::new();

        let valid_trs: Vec<Transaction> = transactions
            .into_iter()
            .filter(|tr| match tr.get_data() {
                Ok(data) => match data {
                    TransactionData::ClassicTransaction {
                        receiver,
                        amount,
                        ..
                    } => {
                        if amount <= 0 as f32 {
                            return false; // Invalid transaction amount
                        }

                        let em = tr.header.get_login();
                        let r = String::from_utf8_lossy(&receiver)
                            .trim_end_matches('\0')
                            .to_string();

                        if em == r {
                            return false; // Cannot send money to yourself
                        }

                        match self.gitlab.check_user_existence(&r) {
                            Ok(exists) => {
                                if !exists {
                                    return false; // Receiver does not exist
                                }
                            }
                            Err(_) => {
                                return false; // Error checking user existence
                            }
                        }

                        let mut be = match balances.get(&em) {
                            Some(b) => *b,
                            None => match self.get_user_balance(&em) {
                                Ok(b) => b,
                                Err(_) => {
                                    return false; // Error getting emitter balance
                                }
                            },
                        };

                        let mut br = match balances.get(&r) {
                            Some(b) => *b,
                            None => match self.get_user_balance(&r) {
                                Ok(b) => b,
                                Err(_) => {
                                    return false; // Error getting receiver balance
                                }
                            },
                        };

                        if (be as i64 - amount as i64) < 0 {
                            false // Insufficient balance
                        } else {
                            be -= amount;
                            br += amount;
                            balances.insert(em, be);
                            balances.insert(r.to_string(), br);
                            true // Valid transaction
                        }
                    }
                    _ => true, // Other transaction types are considered valid
                },
                Err(_) => false, // Error getting transaction data
            })
            .collect();

        if valid_trs.is_empty() {
            println!("No valid transactions found: aborting block");
            return;
        }

        let block = Block::new(self.last_hash, &valid_trs);
        // dbg!(&block);
        self.append(&block);
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.mempool.add(transaction);

        if self.mempool.is_full() {
            println!("Mempool is full, creating a new block");
            self.create_new_block();
        }
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
                                        balance -= amount;
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
