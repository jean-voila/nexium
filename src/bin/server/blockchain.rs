use std::{
    fs::File,
    io::{Read, Write},
};

use json::JsonValue::Null;

const BLOCK_HEADER_SIZE: usize = 82;
const TRANSACTION_COUNT: usize = 10;

#[derive(Debug, Default)]
struct BlockHeader {
    version: u16,
    previous_block_hash: [u8; 32],
    merkle_root: [u8; 32],
    timestamp: u32,
    difficulty_target: u32,
    nonce: u32,
    transactions_size: u32,
}

#[derive(Debug, Default)]
struct TransactionHeader {
    //
}

#[derive(Debug, Default)]
enum DataType {
    #[default]
    Unknow,
}

#[derive(Debug)]
struct Transaction {
    transaction_header: TransactionHeader,
    data: DataType,
    signature: [u8; 256],
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            transaction_header: Default::default(),
            data: Default::default(),
            signature: [0; 256],
        }
    }
}

#[derive(Debug, Default)]
pub struct Block {
    header: BlockHeader,
    transactions: [Transaction; TRANSACTION_COUNT],
}

impl BlockHeader {
    // pub fn new() -> BlockHeader {
    //     BlockHeader {
    //         version: 0,
    //         previous_block_hash: [0; 32],
    //         merkle_root: [0; 32],
    //         timestamp: 0,
    //         bits: 0,
    //         nonce: 0,
    //     }
    // }
}

impl Block {
    // pub fn new() -> Block {
    //     Block {
    //         size: 0,
    //         header: BlockHeader::new(),
    //     }
    // }
}

// impl core::fmt::Display for BlockHeader {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{{\nbits: {}\nmerkle_root: {:?}\nnonce: {}\nprevious_block_hash: {:?}\ntimestamp: {}\nversion: {}\n}}",
//             self.bits,
//             self.merkle_root,
//             self.nonce,
//             self.previous_block_hash,
//             self.timestamp,
//             self.version
//         )
//     }
// }

// impl core::fmt::Debug for Block {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{{\nsize: {}\nheader: {}}}\n", self.size, self.header)
//     }
// }

/// Read a block from the file
/// Returns the new position in the file
pub fn readBlock(file: &mut File, pos: u32) -> u32 {
    let head_buff = &mut [0_u8; BLOCK_HEADER_SIZE];
    let _ = file.read(head_buff);

    let block = Block {
        header: BlockHeader {
            version: u16::from_be_bytes(head_buff[0..2].try_into().unwrap()),
            previous_block_hash: head_buff[2..34].try_into().unwrap(),
            merkle_root: head_buff[34..66].try_into().unwrap(),
            timestamp: u32::from_be_bytes(
                head_buff[66..70].try_into().unwrap(),
            ),
            difficulty_target: u32::from_be_bytes(
                head_buff[70..74].try_into().unwrap(),
            ),
            nonce: u32::from_be_bytes(head_buff[74..78].try_into().unwrap()),
            transactions_size: u32::from_be_bytes(
                head_buff[78..82].try_into().unwrap(),
            ),
        },
        ..Default::default()
    };

    // let tr_buff = [0_u8; block.header.transactions_size as usize];
    let mut tr_buff = &mut vec![0_u8; block.header.transactions_size as usize];
    // let _ = file.read(&mut tr_buff);

    for i in 0..TRANSACTION_COUNT {
        // block.transactions[i].data;
        // block.transactions[i].signature;
        // block.transactions[i].transaction_header;
    }

    // let transactions: [Transaction; TRANSACTION_COUNT] =
    //     [Default::default(); TRANSACTION_COUNT];
    // let mut block = Block {
    //     header: block_header,
    //     // transactions: [Transaction {
    //     //     data: 0,
    //     //     signature: [0; 256],
    //     //     transaction_header: TransactionHeader {

    //     //     },
    //     // }; TRANSACTION_COUNT],
    // };

    // let block: Block = Default::default();
    // block.header.

    return pos + BLOCK_HEADER_SIZE as u32;
}

pub fn writeBlock(file: &mut File, data: &[u8]) {
    let size = data.len() as u32;
    let size_bytes = size.to_be_bytes();
    let _ = file.write(&size_bytes).unwrap();
}
