use super::{
    errors::{BlockchainError, BlockchainFileError},
    iterator::BlockchainIterator,
    iterator_rev::BlockchainIteratorReverse,
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
    rsa::KeyPair,
};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
};

pub struct Blockchain {
    pub balance_cache: HashMap<String, f32>,
    pub hash_cache: HashMap<HeaderPreviousBlockHash, u64>,
    file: File,
    pub last_hash: HeaderPreviousBlockHash,
    mempool: Mempool,
    pub size: u64,
}

impl Blockchain {
    // #[deprecated]
    async fn create_genesis(&mut self, _: &KeyPair) -> Result<(), String> {
        todo!()
        // let t = Transaction::new(
        //     "GENESIS".as_bytes().to_vec(),
        //     0,
        //     "",
        //     DataType::Unknown,
        //     &key,
        // )?;

        // let transactions = vec![t];
        // let block =
        //     Block::new(HeaderPreviousBlockHash::default(), &transactions);
        // self.append(&block)
    }

    fn open_file(rd: bool, wr: bool, cr: bool) -> Result<File, String> {
        let r = OpenOptions::new()
            .read(rd)
            .write(wr)
            .append(wr)
            .create(cr)
            .open(BLOCKCHAIN_FILE);

        let file = r.map_err(|e| {
            format!(
                "Failed to open blockchain file: {}: {}",
                BLOCKCHAIN_FILE, e
            )
        })?;

        Ok(file)
    }

    pub async fn init(key: &KeyPair) -> Result<Self, String> {
        // pub async fn init() -> Result<Self, String> {
        let file = Self::open_file(true, true, true)?;

        let mut b = Self {
            balance_cache: HashMap::new(),
            hash_cache: HashMap::new(),
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

        // read the file and hash the blocks to setup the hash_cache

        if blockchain_size == 0 {
            print!("Blockchain is empty, creating genesis block...");
            b.create_genesis(key).await?;
            print!("\r{:>26}(created genesis block){:>18}", " ", " ");

            // println!("Blockchain is empty, no blocks to read.");
            // return Err("Blockchain is empty".to_string());
        }

        loop {
            if b.size >= blockchain_size {
                break;
            }

            let (buffer, hash) = b.read_block_buffer_at(b.size)?;

            if hash != b.last_hash {
                return Err(BlockchainError::InvalidPreviousHash
                    .as_str()
                    .to_string());
            }

            b.last_hash = Block::double_hash(&buffer);
            b.hash_cache.insert(b.last_hash, b.size);
            b.size += buffer.len() as u64;
        }

        // dbg!(&b.hash_cache);
        // dbg!(&b.hash_cache.len());
        Ok(b)
    }

    pub fn write_block(
        &mut self,
        buff: &Vec<u8>,
        hash: HeaderPreviousBlockHash,
    ) -> Result<(), String> {
        let mut file = Self::open_file(false, true, false)?;

        file.write_all(&buff)
            .map_err(|e| format!("Error writing to file: {}", e))?;

        self.last_hash = hash;
        self.hash_cache.insert(self.last_hash, self.size);
        self.size += buff.len() as u64;

        Ok(())
    }

    pub async fn get_verified_transactions(
        &mut self,
        // trs: &mut Vec<Transaction>,
        gitlab: GitlabClient,
    ) -> (Vec<Transaction>, HashMap<String, f32>) {
        let mut trs = self.mempool.dump();
        trs.sort_by(|a, b| a.header.timestamp.cmp(&b.header.timestamp));

        let mut balances: HashMap<String, f32> = HashMap::new();
        let mut valid_trs: Vec<Transaction> = vec![];

        for tr in trs.iter() {
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

                        match gitlab.check_user_existence_async(&r).await {
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
                            None => match self.calc_user_balance(&em) {
                                Ok(b) => b,
                                Err(_) => {
                                    continue; // Error getting emitter balance
                                }
                            },
                        };

                        let mut br = match balances.get(&r) {
                            Some(b) => *b,
                            None => match self.calc_user_balance(&r) {
                                Ok(b) => b,
                                Err(_) => {
                                    continue; // Error getting receiver balance
                                }
                            },
                        };

                        if (be as i64 - amount as i64) < 0 {
                            continue; // Insufficient balance
                        } else {
                            be -= amount;
                            br += amount;
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

        (valid_trs, balances)
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> bool {
        self.mempool.add(transaction);
        self.mempool.is_full()
    }

    pub fn update_balances(&mut self, balances: HashMap<String, f32>) {
        for (login, balance) in balances {
            self.balance_cache.insert(login, balance);
        }
    }

    pub fn read_block_buffer(
        &self,
        file: &mut File,
    ) -> Result<(Vec<u8>, HeaderPreviousBlockHash), String> {
        let mut header_buff = [0_u8; BLOCK_HEADER_SIZE];
        file.read_exact(&mut header_buff)
            .map_err(|e| BlockchainFileError::FailedToRead(e).to_string())?;

        let header = BlockHeader::from_buff(&header_buff);
        let mut buff =
            vec![0_u8; BLOCK_HEADER_SIZE + header.transactions_size as usize];
        buff[0..BLOCK_HEADER_SIZE].copy_from_slice(&header_buff);

        file.read_exact(&mut buff[BLOCK_HEADER_SIZE..])
            .map_err(|e| BlockchainFileError::FailedToRead(e).to_string())?;
        Ok((buff, header.previous_block_hash))
    }

    pub fn read_block_buffer_at(
        &self,
        offset: u64,
    ) -> Result<(Vec<u8>, HeaderPreviousBlockHash), String> {
        let mut file = Self::open_file(true, false, false)?;
        self.read_block_buffer_file_at(&mut file, offset)
    }

    pub fn read_block_buffer_file_at(
        &self,
        file: &mut File,
        offset: u64,
    ) -> Result<(Vec<u8>, HeaderPreviousBlockHash), String> {
        file.seek(SeekFrom::Start(offset))
            .map_err(|e| BlockchainFileError::FailedToSeek(e).to_string())?;

        self.read_block_buffer(file)
    }

    pub fn read_block_file_at(
        &self,
        file: &mut File,
        offset: u64,
    ) -> Result<Block, String> {
        let (buff, _) = self.read_block_buffer_file_at(file, offset)?;
        Block::from_buffer(&buff)
    }

    pub fn get_block_file_from_hash(
        &self,
        file: &mut File,
        hash: &HeaderPreviousBlockHash,
    ) -> Result<Block, String> {
        match self.hash_cache.get(hash) {
            Some(offset) => self.read_block_file_at(file, *offset),
            None => Err(BlockchainError::HashNotFound.as_str().to_string()),
        }
    }

    fn calc_user_balance<T>(&self, login: T) -> Result<f32, String>
    where
        T: AsRef<str>,
    {
        let log = login.as_ref();
        let mut log_bytes = [0; TRANSACTION_RECEIVER];
        log_bytes[..log.len()].copy_from_slice(log.as_bytes());
        let mut balance = INITIAL_BALANCE as f32;

        for block in self.iter()? {
            for tr in block?.transactions {
                if tr.header.data_type != DataType::ClassicTransaction {
                    continue; // Skip non-classic transactions
                }

                let data = tr.get_data();
                match data {
                    Ok(TransactionData::ClassicTransaction {
                        receiver,
                        amount,
                        ..
                    }) => {
                        if log_bytes == receiver {
                            balance += amount;
                        } else if log_bytes == tr.header.emitter {
                            balance -= amount;
                        };
                    }
                    Ok(_) => continue, // skip other transaction types
                    Err(e) => {
                        return Err(format!(
                            "Failed to get transaction data: {}",
                            e
                        ));
                    }
                }
            }
        }

        Ok(balance)
    }

    pub fn update_balance_cache<T>(&mut self, login: T) -> Result<f32, String>
    where
        T: AsRef<str>,
    {
        let log = login.as_ref();
        let balance = self.calc_user_balance(log)?;
        self.balance_cache.insert(log.to_string(), balance);
        Ok(balance)
    }

    pub fn get_balance_cache<T>(&mut self, login: T) -> Option<f32>
    where
        T: AsRef<str>,
    {
        let login = login.as_ref();
        self.balance_cache.get(login).cloned()
    }

    pub fn get_balance<T>(&mut self, login: T) -> Result<f32, String>
    where
        T: AsRef<str>,
    {
        if let Some(balance) = self.get_balance_cache(&login) {
            return Ok(balance);
        }
        self.update_balance_cache(&login)
    }

    pub fn iter(&self) -> Result<BlockchainIterator, String> {
        Ok(BlockchainIterator {
            blockchain: &self,
            file: Self::open_file(true, false, false)?,
            offset: 0,
        })
    }

    pub fn iter_rev(&self) -> Result<BlockchainIteratorReverse, String> {
        Ok(BlockchainIteratorReverse {
            blockchain: &self,
            file: Self::open_file(true, false, false)?,
            hash: self.last_hash,
        })
    }
}
