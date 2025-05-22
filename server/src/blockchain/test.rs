use super::{
    blockchain::Blockchain,
    mempool::Mempool,
    structure::{
        block::Block,
        consts::{HEADER_MERKLE_ROOT_SIZE, HEADER_PREVIOUS_BLOCK_HASH_SIZE},
    },
};
use nexium::{
    blockchain::{
        consts::SIGNATURE_SIZE, data_type::DataType, transaction::Transaction,
    },
    defaults::KEYPAIR_BIT_SIZE,
    rsa::KeyPair,
};

pub fn main() {
    let mut blockchain =
        Blockchain::init().expect("Failed to initialize blockchain");
    return;
    let prev_hash = ['a' as u8; HEADER_PREVIOUS_BLOCK_HASH_SIZE];

    println!("Key creation");
    let key = KeyPair::generate(KEYPAIR_BIT_SIZE, "");
    println!("Key created");

    let tr1 = match Transaction::new(
        [2; 1623].to_vec(),
        1862,
        "william.valenduc",
        DataType::ClassicTransaction,
        &key,
    ) {
        Ok(tr) => tr,
        Err(e) => {
            println!("Error creating transaction: {}", e);
            return;
        }
    };

    let tr2 = match Transaction::new(
        [1; 2863].to_vec(),
        999,
        "jean.herail",
        DataType::Unknown,
        &key,
    ) {
        Ok(tr) => tr,
        Err(e) => {
            println!("Error creating transaction: {}", e);
            return;
        }
    };

    // let transactions = vec![tr1, tr2];
    // let block = Block::new(1, prev_hash, 3, 19, transactions);
    // dbg!(&block);

    // let buff = block.clone().to_buffer();
    // std::fs::write("block.bin", &buff).expect("Unable to write file");

    // // read the file
    // let buff_r = std::fs::read("block.bin").expect("Unable to read file");
    // let block_r = match Block::from_buffer(&buff_r) {
    //     Ok(block) => block,
    //     Err(e) => {
    //         println!("Error creating block from buffer: {}", e);
    //         return;
    //     }
    // };

    // let same_buff = buff == buff_r;
    // println!("Buffers are the same: {}", same_buff);

    // let same_block = block == block_r;
    // println!("Blocks are the same: {}", same_block);

    // dbg!(block_r);

    blockchain.add_transaction(tr1);
    blockchain.add_transaction(tr2);
}
