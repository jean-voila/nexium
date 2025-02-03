use nexium::rsa;
mod config;
mod srv_network;

fn main() {
    let keypair = rsa::KeyPair::generate(2048);
    dbg!(keypair);
}
