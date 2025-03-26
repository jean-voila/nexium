use std::{
    fs::File,
    io::{Read, Write},
};

const BLOCK_HEADER_SIZE: usize = 82;
const HEADER_VERSION: usize = 2;
const HEADER_PREVIOUS_BLOCK_HASH: usize = 32;
const HEADER_MERKLE_ROOT: usize = 32;
const HEADER_TIMESTAMP: usize = 4;
const HEADER_DIFFICULTY_TARGET: usize = 4;
const HEADER_NONCE: usize = 4;
const HEADER_TRANSACTION_SIZE: usize = 4;

const TRANSACTION_COUNT: usize = 10; //
const TRANSACTION_HEADER_SIZE: usize = 73;
const TRANSACTION_SIZE: usize = 2;
const TRANSACTION_TIMESTAMP: usize = 4;
const TRANSACTION_FEES: usize = 2;
const TRANSACTION_EMITTER: usize = 64;
const TRANSACTION_DATA_TYPE: usize = 1;
const SIGNATURE_SIZE: usize = 256;

#[derive(Debug, Default, Clone, Copy)]
struct BlockHeader {
    version: u16,
    previous_block_hash: [u8; HEADER_PREVIOUS_BLOCK_HASH],
    merkle_root: [u8; HEADER_MERKLE_ROOT],
    timestamp: u32,
    difficulty_target: u32,
    nonce: u32,
    transactions_size: u32,
}

impl BlockHeader {
    fn from_buff(buff: [u8; BLOCK_HEADER_SIZE]) -> Self {
        BlockHeader {
            version: u16::from_be_bytes(buff[0..2].try_into().unwrap()),
            previous_block_hash: buff[2..34].try_into().unwrap(),
            merkle_root: buff[34..66].try_into().unwrap(),
            timestamp: u32::from_be_bytes(buff[66..70].try_into().unwrap()),
            difficulty_target: u32::from_be_bytes(
                buff[70..74].try_into().unwrap(),
            ),
            nonce: u32::from_be_bytes(buff[74..78].try_into().unwrap()),
            transactions_size: u32::from_be_bytes(
                buff[78..82].try_into().unwrap(),
            ),
        }
    }

    fn to_buffer(self) -> [u8; BLOCK_HEADER_SIZE] {
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

#[derive(Debug, Default, Clone, Copy)]
enum DataType {
    #[default]
    Unknown = 0,
    ClassicTransaction = 1,
}

impl DataType {
    fn from_u8(t: u8) -> Self {
        match t {
            1 => DataType::ClassicTransaction,
            _ => DataType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct TransactionHeader {
    transaction_size: u16,
    timestamp: u32,
    fees: u16,
    emitter: [u8; TRANSACTION_EMITTER],
    data_type: DataType,
}

#[derive(Debug, Clone, Copy)]
struct Transaction {
    transaction_header: TransactionHeader,
    // data: DataType,
    signature: [u8; SIGNATURE_SIZE],
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            signature: [0; SIGNATURE_SIZE],
            ..Default::default()
        }
    }
}

impl Default for TransactionHeader {
    fn default() -> Self {
        Self {
            emitter: [0; TRANSACTION_EMITTER],
            ..Default::default()
        }
    }
}

impl Transaction {
    fn fill_from_buffer(&mut self, buff: &[u8]) -> usize {
        self.transaction_header.fill_from_buffer(
            &buff[0..TRANSACTION_HEADER_SIZE].try_into().unwrap(),
        );
        // self.data = DataType::Unknown;
        self.signature = buff
            [self.transaction_header.transaction_size as usize..]
            .try_into()
            .unwrap();
        TRANSACTION_HEADER_SIZE
            + self.transaction_header.transaction_size as usize
    }

    fn to_buffer(self) -> Vec<u8> {
        let mut res =
            vec![0; self.transaction_header.transaction_size as usize];
        res[0..TRANSACTION_HEADER_SIZE]
            .copy_from_slice(&self.transaction_header.to_buffer());
        res[TRANSACTION_HEADER_SIZE..].copy_from_slice(&self.signature);
        return res;
    }
}

impl TransactionHeader {
    fn fill_from_buffer(&mut self, buff: &[u8; TRANSACTION_HEADER_SIZE]) {
        self.transaction_size =
            u16::from_be_bytes(buff[0..2].try_into().unwrap());
        self.timestamp = u32::from_be_bytes(buff[2..6].try_into().unwrap());
        self.fees = u16::from_be_bytes(buff[6..8].try_into().unwrap());
        self.emitter = buff[8..72].try_into().unwrap();
        self.data_type = DataType::from_u8(buff[72]);
    }

    fn to_buffer(self) -> [u8; TRANSACTION_HEADER_SIZE] {
        let mut res = [0; TRANSACTION_HEADER_SIZE];
        res[0..2].copy_from_slice(&self.transaction_size.to_be_bytes());
        res[2..6].copy_from_slice(&self.timestamp.to_be_bytes());
        res[6..8].copy_from_slice(&self.fees.to_be_bytes());
        res[8..72].copy_from_slice(&self.emitter);
        res[72] = self.data_type as u8;
        return res;
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Block {
    header: BlockHeader,
    transactions: [Transaction; TRANSACTION_COUNT],
}

// impl Block {
//     fn to_buffer(self) -> Vec<u8> {
//         let mut res: Vec<u8> =
//             vec![0; BLOCK_HEADER_SIZE + self.header.transactions_size as usize];

//         return res;
//     }
// }

/// Read a block from the file
/// Returns the new position in the file
pub fn readBlock(file: &mut File, pos: u32) -> (Block, u32) {
    let head_buff = &mut [0_u8; BLOCK_HEADER_SIZE];
    let _ = file.read(head_buff);

    let mut block = Block {
        header: BlockHeader::from_buff(*head_buff),
        ..Default::default()
    };

    let mut tr_buff = &mut vec![0_u8; block.header.transactions_size as usize];
    let _ = file.read(&mut tr_buff);

    let mut current_pos = 0;

    for i in 0..TRANSACTION_COUNT {
        current_pos +=
            block.transactions[i].fill_from_buffer(&tr_buff[current_pos..]);
    }

    (
        block,
        pos + BLOCK_HEADER_SIZE as u32 + block.header.transactions_size as u32,
    )
}

pub fn writeBlock(file: &mut File, block: Block) {
    let head_buff = block.header.to_buffer();
    let _ = file.write(&head_buff).unwrap();

    let mut tr_buff = vec![];
    for i in 0..TRANSACTION_COUNT {
        tr_buff.extend(block.transactions[i].to_buffer());
    }
    let _ = file.write(&tr_buff).unwrap();
}
