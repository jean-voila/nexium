use crate::utils::time::current_time;

use super::consts::{
    BLOCK_HEADER_SIZE, HEADER_MERKLE_ROOT_SIZE,
    HEADER_PREVIOUS_BLOCK_HASH_SIZE, HEADER_VERSION,
};

pub type HeaderMerkleRoot = [u8; HEADER_MERKLE_ROOT_SIZE];
pub type HeaderPreviousBlockHash = [u8; HEADER_PREVIOUS_BLOCK_HASH_SIZE];

// #[derive(Debug, Default, Clone, Copy)]
#[derive(Default, Clone, Copy, PartialEq)]
pub struct BlockHeader {
    version: u16,
    previous_block_hash: HeaderPreviousBlockHash,
    merkle_root: HeaderMerkleRoot,
    timestamp: u32,
    difficulty_target: u32,
    nonce: u32,
    pub transactions_size: u32,
}

impl BlockHeader {
    pub fn new(
        version: u16,
        previous_block_hash: HeaderPreviousBlockHash,
        merkle_root: HeaderMerkleRoot,
        difficulty_target: u32,
        nonce: u32,
        transactions_size: u32,
    ) -> Self {
        Self {
            version,
            previous_block_hash,
            merkle_root,
            timestamp: current_time(),
            difficulty_target,
            nonce,
            transactions_size,
        }
    }

    pub fn from_buff(buff: [u8; BLOCK_HEADER_SIZE]) -> Self {
        let pbh_start = HEADER_VERSION;
        let mr_start = pbh_start + HEADER_PREVIOUS_BLOCK_HASH_SIZE;
        let tt_start = mr_start + HEADER_MERKLE_ROOT_SIZE;
        let dt_start = tt_start + 4;
        let nonce_start = dt_start + 4;
        let ts_start = nonce_start + 4;
        let end = ts_start + 4; // HEADER_SIZE
                                // let end = BLOCK_HEADER_SIZE;
        BlockHeader {
            version: u16::from_be_bytes(buff[0..pbh_start].try_into().unwrap()),
            previous_block_hash: buff[pbh_start..mr_start].try_into().unwrap(),
            merkle_root: buff[mr_start..tt_start].try_into().unwrap(),
            timestamp: u32::from_be_bytes(
                buff[tt_start..dt_start].try_into().unwrap(),
            ),
            difficulty_target: u32::from_be_bytes(
                buff[dt_start..nonce_start].try_into().unwrap(),
            ),
            nonce: u32::from_be_bytes(
                buff[nonce_start..ts_start].try_into().unwrap(),
            ),
            transactions_size: u32::from_be_bytes(
                buff[ts_start..end].try_into().unwrap(),
            ),
        }
        // BlockHeader {
        //     version: u16::from_be_bytes(
        //         buff[0..HEADER_VERSION].try_into().unwrap(),
        //     ),
        //     previous_block_hash: buff[2..34].try_into().unwrap(),
        //     merkle_root: buff[34..66].try_into().unwrap(),
        //     timestamp: u32::from_be_bytes(buff[66..70].try_into().unwrap()),
        //     difficulty_target: u32::from_be_bytes(
        //         buff[70..74].try_into().unwrap(),
        //     ),
        //     nonce: u32::from_be_bytes(buff[74..78].try_into().unwrap()),
        //     transactions_size: u32::from_be_bytes(
        //         buff[78..82].try_into().unwrap(),
        //     ),
        // }
    }

    pub fn to_buffer(self) -> [u8; BLOCK_HEADER_SIZE] {
        let mut res = [0; BLOCK_HEADER_SIZE];
        res[0..2].copy_from_slice(&self.version.to_be_bytes());
        res[2..34].copy_from_slice(&self.previous_block_hash);
        res[34..66].copy_from_slice(&self.merkle_root);
        res[66..70].copy_from_slice(&self.timestamp.to_be_bytes());
        res[70..74].copy_from_slice(&self.difficulty_target.to_be_bytes());
        res[74..78].copy_from_slice(&self.nonce.to_be_bytes());
        res[78..82].copy_from_slice(&self.transactions_size.to_be_bytes());
        return res;
    }
}

impl core::fmt::Debug for BlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;
        write!(f, "  transactions_size: {},\n", self.transactions_size)?;
        write!(f, "  version: {},\n", self.version)?;
        write!(
            f,
            "  previous_block_hash: {:?},\n",
            String::from_utf8(self.previous_block_hash.to_vec()).unwrap()
        )?;
        write!(
            f,
            "  merkle_root: {:?},\n",
            String::from_utf8(self.merkle_root.to_vec()).unwrap()
        )?;
        write!(f, "  timestamp: {},\n", self.timestamp)?;
        write!(f, "  difficulty_target: {},\n", self.difficulty_target)?;
        write!(f, "  nonce: {},\n", self.nonce)?;
        write!(f, "}}")?;
        Ok(())
    }
}
