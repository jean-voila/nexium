// use json;
// use rusqlite::Connection;
// use std::path::Path;
use rsa;

const _CONF_FILE: &str = "config/nexium.json";

const _DEF_PORT: u16 = 4343;
const _DEF_DB_FP: &str = "blockchain.db";

const _DEBUG: bool = true;

struct _Config {
    port: String,
    db_fp: String,
    keys_fp: String,
}

fn main() {
    // let conn = Connection::open(DB_PATH);

    let keypair = rsa::KeyPair::generate(2048);

    keypair.debug_print();
}
