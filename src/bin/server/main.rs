mod config;
mod srv_network;

use config::Config;
use nexium::rsa;
use std::path::Path;

fn main() {
    /////////////////////////
    // rsa test
    // let keypair = rsa::KeyPair::generate(2048);
    // dbg!(keypair);
    /////////////////////////

    /////////////////////////
    // config test
    let p = Path::new("./test_data/config.json");
    // let config: Config = config::Config::new();
    // config.to_file(p);

    let cfg = Config::from_file(p);
    dbg!(cfg);
    /////////////////////////
}
