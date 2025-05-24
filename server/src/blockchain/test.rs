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

const KEY_FILE: &str = ".nexiumlocal/private-key.pem";

pub fn main() {
    let key = KeyPair::priv_from_file(KEY_FILE, "william.valenduc", "")
        .expect("Failed to load private key from file");

    let mut bc = Blockchain::init().expect("Failed to initialize blockchain");
    // let b1 = match bc.read_block(0) {
    //     Ok(b) => b,
    //     Err(e) => {
    //         println!("Error getting block: {}", e);
    //         return;
    //     }
    // };
    // dbg!(&b1);
    dbg!(hex::encode(&bc.last_hash));
    // return;

    // let b2 = match bc.get_block(&bc.last_hash.clone()) {
    //     Ok(b) => b,
    //     Err(e) => {
    //         println!("Error getting block: {}", e);
    //         return;
    //     }
    // };
    // dbg!(&b2);
    // dbg!(&b1 == &b2);
    // return;

    let login1 = "william.valenduc";
    let login2 = "jean.herail";
    let balance = bc
        .get_user_balance(login1)
        .expect("Failed to get user balance");
    println!("Balance: {} {}", login1, balance);

    let balance = bc
        .get_user_balance(login2)
        .expect("Failed to get user balance");
    println!("Balance: {} {}", login2, balance);
    return;

    //////////////////

    // let t = Transaction::new(
    //     "GENESIS".as_bytes().to_vec(),
    //     0,
    //     "william.valenduc",
    //     DataType::Unknown,
    //     &key,
    // )
    // .expect("Failed to create transaction");
    // dbg!(&t);

    // bc.add_transaction(t);
    // return;

    ///////////////

    let tr = Transaction::new_classic(
        "jean.herail",
        100.65489 as f32,
        "",
        0,
        "william.valenduc",
        &key,
    );

    // let tr = Transaction::new_classic(
    //     "william.valenduc",
    //     50,
    //     "TEST_ERROR",
    //     0,
    //     "jean.herail",
    //     &key,
    // );

    // let tr = Transaction::new(
    //     "This is a test".as_bytes().to_vec(),
    //     0,
    //     "william.valenduc",
    //     DataType::Unknown,
    //     &key,
    // );

    let tr1 = match tr {
        Ok(t) => t,
        Err(e) => {
            println!("Error creating transaction: {}", e);
            return;
        }
    };
    // dbg!(&tr1);
    let json =
        serde_json::to_string(&tr1).expect("Failed to serialize transaction");
    // println!("Transaction: {}", json);
    let crypt = key.crypt_split(&json).expect("Failed to encrypt");
    println!("Crypt:\n{}", crypt);
    // bc.add_transaction(tr1);
}
