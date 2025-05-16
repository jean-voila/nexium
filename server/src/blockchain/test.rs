use super::structure::{
    block::Block,
    consts::{
        HEADER_MERKLE_ROOT_SIZE, HEADER_PREVIOUS_BLOCK_HASH_SIZE,
        SIGNATURE_SIZE,
    },
    data_type::DataType,
    transaction::Transaction,
};

pub fn main() {
    let emitter1 = ['w' as u8; 64];
    let emitter2 = ['y' as u8; 64];
    let signature1 = ['z' as u8; SIGNATURE_SIZE];
    let signature2 = ['p' as u8; SIGNATURE_SIZE];
    let prev_hash = ['a' as u8; HEADER_PREVIOUS_BLOCK_HASH_SIZE];
    let merkle_root = ['b' as u8; HEADER_MERKLE_ROOT_SIZE];

    let transactions = vec![
        Transaction::new(
            [2; 1623].to_vec(),
            1862,
            emitter1,
            DataType::ClassicTransaction,
            signature1,
        ),
        Transaction::new(
            [2; 2863].to_vec(),
            999,
            emitter2,
            DataType::Unknown,
            signature2,
        ),
    ];
    let block = Block::new(1, prev_hash, merkle_root, 3, 19, transactions);
    // dbg!(&block);

    let buff = block.clone().to_buffer();
    std::fs::write("block.bin", &buff).expect("Unable to write file");

    // read the file
    let buff_r = std::fs::read("block.bin").expect("Unable to read file");
    let block_r = Block::from_buffer(&buff_r);

    let same_buff = buff == buff_r;
    println!("Buffers are the same: {}", same_buff);

    let same_block = block == block_r;
    println!("Blocks are the same: {}", same_block);

    // dbg!(block_r);
}
