use std::{
    fs::File,
    io::{Read, Write},
};

const BLOCK_HEADER_SIZE: usize = 82;
const TRANSACTION_COUNT: usize = 10;
const TRANSACTION_HEADER_SIZE: usize = 73;

#[derive(Debug, Default, Clone, Copy)]
struct BlockHeader {
    version: u16,
    previous_block_hash: [u8; 32],
    merkle_root: [u8; 32],
    timestamp: u32,
    difficulty_target: u32,
    nonce: u32,
    transactions_size: u32,
}

impl BlockHeader {
    fn from_buff(buff: [u8; 82]) -> Self {
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
    emitter: [u8; 64],
    data_type: DataType,
}

#[derive(Debug, Clone, Copy)]
struct Transaction {
    transaction_header: TransactionHeader,
    // data: DataType,
    signature: [u8; 256],
}

impl DataType {
    fn from(t: u8) {}
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            signature: [0; 256],
            ..Default::default()
        }
    }
}

impl Default for TransactionHeader {
    fn default() -> Self {
        Self {
            emitter: [0; 64],
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
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Block {
    header: BlockHeader,
    transactions: [Transaction; TRANSACTION_COUNT],
}

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

pub fn writeBlock(file: &mut File, data: &[u8]) {
    let size = data.len() as u32;
    let size_bytes = size.to_be_bytes();
    let _ = file.write(&size_bytes).unwrap();
}
