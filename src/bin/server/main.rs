mod blockchain;
mod config;
//mod srv_network;

use config::generate_config;
use nexium::rsa;
// use nexium::sha256;
use std::env;

const GEN_CONFIG_ARG: &str = "--generate-config";

fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.contains(&GEN_CONFIG_ARG.to_string()) {
    //     generate_config();
    //     return;
    // }
    let keypair: rsa::KeyPair = rsa::KeyPair::generate(2048);
    dbg!(keypair);

    /////////////////////////
    // rsa test
    // let keypair = rsa::KeyPair::generate(2048);
    // dbg!(keypair);
    /////////////////////////

    /////////////////////////
    // config test
    // let p = Path::new("./test_data/config.json");
    // let config: Config = config::Config::new();
    // config.to_file(p);

    // let cfg = Config::from_file(p);
    // dbg!(cfg);
    /////////////////////////

    // let string_test: String = String::from("je hais ce monde");
    // dbg!(string_test.clone());

    // let preprocessed = sha256::preprocessing(string_test.clone());
    // dbg!(preprocessed.clone());

    // let processed = sha256::processing(preprocessed.clone());
    // dbg!(processed.clone());
}
