use std::{
    fs::File,
    io::{Read, Write},
};

struct BlockHeader {
    version: u16,
    previous_block_hash: [u8; 32],
    merkle_root: [u8; 32],
    timestamp: u32,
    difficulty_target: u32,
    nonce: u32,
    transaction: u16,
}

pub struct Block {
    size: u32,
    header: BlockHeader,
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
    let size_buff = &mut [0_u8; 4];
    let _ = file.read(size_buff);
    let head_buff = &mut [0_u8; 80];
    let _ = file.read(head_buff);

    let block = Block {
        size: u32::from_be_bytes(*size_buff),
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
            transaction: u16::from_be_bytes(
                head_buff[78..80].try_into().unwrap(),
            ),
        },
    };

    return pos + block.size;
}

pub fn writeBlock(file: &mut File, data: &[u8]) {
    let size = data.len() as u32;
    let size_bytes = size.to_be_bytes();
    let _ = file.write(&size_bytes).unwrap();
}
