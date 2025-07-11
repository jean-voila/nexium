use super::{block_header::BlockHeader, consts::BLOCK_HEADER_SIZE};
use nexium::{
    blockchain::transaction::{
        transactions_vec_buff,
        // transactions_vec_size,
        Transaction,
    },
    defaults::{BLOCK_VERSION, DIFFICULTY_TARGET},
    sha256::sha256,
    types::{DoubleHash, Hash, DOUBLE_HASH_SIZE, HASH_SIZE},
};

#[derive(Default, PartialEq)]
pub struct Block {
    pub header: BlockHeader,
    // pub transactions: [Transaction; TRANSACTION_COUNT],
    pub transactions: Vec<Transaction>,
}

impl Block {
    // fn merkle_root_hash<T>(tr1: T, tr2: T) -> Vec<u8>
    fn merkle_root_hash<T>(tr1: T, tr2: T) -> DoubleHash
    where
        T: AsRef<[u8]>,
    {
        // let mut res = Vec::with_capacity(DOUBLE_HASH_SIZE);
        // res.extend_from_slice(&sha256(tr1));
        // res.extend_from_slice(&sha256(tr2));

        let mut res = [0_u8; DOUBLE_HASH_SIZE];
        res[0..HASH_SIZE].copy_from_slice(&sha256(tr1));
        res[HASH_SIZE..].copy_from_slice(&sha256(tr2));
        return res;
    }

    fn merkle_root(trs: &Vec<Vec<u8>>) -> Hash {
        let hashed_trs: Vec<Hash> = trs.iter().map(|tr| sha256(tr)).collect();

        let mut concat_hashes: Vec<DoubleHash> = hashed_trs
            .chunks(2)
            .map(|pair| {
                if pair.len() == 2 {
                    Block::merkle_root_hash(
                        pair[0].as_slice(),
                        pair[1].as_slice(),
                    )
                } else {
                    Block::merkle_root_hash(
                        pair[0].as_slice(),
                        pair[0].as_slice(),
                    )
                }
            })
            .collect();

        while concat_hashes.len() > 1 {
            concat_hashes = concat_hashes
                .chunks(2)
                .map(|pair| {
                    if pair.len() == 2 {
                        Block::merkle_root_hash(pair[0], pair[1])
                    } else {
                        Block::merkle_root_hash(pair[0], pair[0])
                    }
                })
                .collect();
        }

        sha256(&concat_hashes[0])
    }

    pub fn create(
        previous_block_hash: Hash,
        transactions: Vec<Transaction>,
    ) -> (Vec<u8>, Hash) {
        let trs_buffered = transactions_vec_buff(transactions);
        let merkle_root = Block::merkle_root(&trs_buffered);
        let trs_buff = trs_buffered.concat();
        let size = trs_buff.len() as u32;
        let mut hash;
        let mut nonce = 0;

        // let mut buff = vec![0_u8; BLOCK_HEADER_SIZE + size as usize];
        // buff[BLOCK_HEADER_SIZE..].copy_from_slice(&trs_buff);

        let mut buff = Vec::with_capacity(BLOCK_HEADER_SIZE + size as usize);
        buff.resize(BLOCK_HEADER_SIZE, 0);
        buff.extend_from_slice(&trs_buff);

        let mut target = String::new();
        for _ in 0..DIFFICULTY_TARGET {
            target.push('0');
        }

        loop {
            let header = BlockHeader::new(
                BLOCK_VERSION,
                previous_block_hash,
                merkle_root,
                DIFFICULTY_TARGET,
                nonce,
                size,
            );

            let head_buff = header.to_buffer();
            buff[0..BLOCK_HEADER_SIZE].copy_from_slice(&head_buff);

            hash = Block::double_hash(&buff);
            let h = hex::encode(&hash);
            if h.starts_with(&target) {
                break;
            }
            nonce += 1;
        }

        (buff, hash)
    }

    pub fn double_hash<T>(hash: T) -> Hash
    where
        T: AsRef<[u8]>,
    {
        sha256(&sha256(&hash))
    }

    // pub fn size(&self) -> u32 {
    //     BLOCK_HEADER_SIZE as u32 + transactions_vec_size(&self.transactions)
    // }

    pub fn from_buffer(buff: &[u8]) -> Result<Self, String> {
        let header_buff: [u8; BLOCK_HEADER_SIZE] =
            match buff[0..BLOCK_HEADER_SIZE].try_into() {
                Ok(h) => h,
                Err(_) => {
                    return Err("Failed to read block header".to_string());
                }
            };
        let header = BlockHeader::from_buff(&header_buff);

        let transactions_size = header.transactions_size as usize;

        // check buffer size
        // dbg!(buff.len(), transactions_size + BLOCK_HEADER_SIZE);
        // if buff.len() != transactions_size + BLOCK_HEADER_SIZE {
        //     panic!("Buffer size is not correct");
        // }

        let mut transaction;
        let mut offset = BLOCK_HEADER_SIZE;
        let mut transactions = vec![];
        while offset < transactions_size {
            transaction = match Transaction::from_buffer(&buff[offset..]) {
                Ok(t) => t,
                Err(_) => {
                    return Err(format!(
                        "Error while reading transaction at offset {}",
                        offset
                    ));
                }
            };
            offset += transaction.size() as usize;
            transactions.push(transaction);
        }

        Ok(Self {
            header,
            transactions,
        })
    }

    // fn transactions_buffer(trs: Vec<Transaction>) -> Vec<u8> {
    //     let size = transactions_vec_size(&trs);
    //     let mut offset = 0;
    //     let mut res = vec![0_u8; size as usize];

    //     for t in trs {
    //         let buff = t.to_buffer();
    //         res[offset..offset + buff.len()].copy_from_slice(&buff);
    //         offset += buff.len();
    //     }
    //     return res;
    // }

    // pub fn to_buffer(&self) -> Vec<u8> {
    //     let mut res = vec![];
    //     res.extend_from_slice(&self.header.clone().to_buffer());
    //     res.extend_from_slice(Self::transactions_buffer(self.transactions));
    //     return res;
    // }
}

impl core::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;

        write!(f, "header: {:?},\n", self.header)?;
        write!(f, "transactions: [\n")?;
        for t in &self.transactions {
            write!(f, "{:?},\n", t)?;
        }
        write!(f, "],\n")?;
        write!(f, "}}")?;
        Ok(())
    }
}
