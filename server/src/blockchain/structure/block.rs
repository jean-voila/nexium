use super::{
    block_header::{BlockHeader, HeaderMerkleRoot, HeaderPreviousBlockHash},
    consts::BLOCK_HEADER_SIZE,
};
use nexium::{
    blockchain::transaction::{transaction_vec_size, Transaction},
    sha256::sha256,
};

#[derive(Default, Clone, PartialEq)]
pub struct Block {
    pub header: BlockHeader,
    // pub transactions: [Transaction; TRANSACTION_COUNT],
    pub transactions: Vec<Transaction>,
}

impl Block {
    fn merkle_root_hash(tr1: &Vec<u8>, tr2: &Vec<u8>) -> Vec<u8> {
        let mut res = vec![0; 64];
        res[0..32].copy_from_slice(&sha256(tr1));
        res[32..64].copy_from_slice(&sha256(tr2));
        return res;
    }

    // fn merkle_root_rec(trs: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    //     if trs.len() == 1 {
    //         return trs;
    //     };

    //     let res = trs
    //         .chunks(2)
    //         .map(|pair| {
    //             if pair.len() == 2 {
    //                 Block::merkle_root_hash(&pair[0], &pair[1])
    //             } else {
    //                 Block::merkle_root_hash(&pair[0], &pair[0])
    //             }
    //         })
    //         .collect();

    //     Block::merkle_root_rec(res)
    // }

    fn merkle_root(transactions: &Vec<Transaction>) -> HeaderMerkleRoot {
        let mut trs: Vec<Vec<u8>> =
            transactions.iter().map(|tr| tr.to_buffer()).collect();

        while trs.len() > 1 {
            trs = trs
                .chunks(2)
                .map(|pair| {
                    if pair.len() == 2 {
                        Block::merkle_root_hash(&pair[0], &pair[1])
                    } else {
                        Block::merkle_root_hash(&pair[0], &pair[0])
                    }
                })
                .collect();
        }

        // let mut res = [0; 32];
        // res.copy_from_slice(&trs[0]);
        // return res;
        sha256(&trs[0])
    }

    pub fn new(
        version: u16,
        previous_block_hash: HeaderPreviousBlockHash,
        difficulty_target: u32,
        nonce: u32,
        transactions: Vec<Transaction>,
    ) -> Self {
        let size = transaction_vec_size(&transactions);
        Self {
            header: BlockHeader::new(
                version,
                previous_block_hash,
                Block::merkle_root(&transactions),
                difficulty_target,
                nonce,
                size,
            ),
            transactions,
        }
    }

    // pub fn size(&self) -> u32 {
    //     BLOCK_HEADER_SIZE as u32 + transaction_vec_size(&self.transactions)
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

    pub fn to_buffer(self) -> Vec<u8> {
        let mut res = vec![];
        res.extend_from_slice(&self.header.to_buffer());
        for t in self.transactions {
            res.extend_from_slice(&t.to_buffer());
        }
        return res;
    }
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
