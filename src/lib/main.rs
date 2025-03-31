pub mod gitlab;
pub mod login;
pub mod rsa;
pub mod sha256;

fn main() {
    let key_pair = KeyPair {
        bit_length: 2048,
        n: BigUint::parse_bytes(b"C4F1B7A3D9E0F", 16).unwrap(),
        e: BigUint::from(65537u32),
        d: BigUint::parse_bytes(b"5A3D2E1C4B0F", 16).unwrap(),
        p: BigUint::parse_bytes(b"F7E75FDC469067FFDC4E847C51F452DF", 16).unwrap(),
        q: BigUint::parse_bytes(b"E85CED54AF57E53E092113E62F436F4F", 16).unwrap(),
    };

    let message: &[u8] = b"Hello, RSA secured signing!";

    match message_sign(message, &key_pair) {
        Ok(signature) => {
            println!("normalement: {}", signature);

            match signature_verification(message, &signature, &key_pair) {
                Ok(true) => println!("Ok"),
                Ok(false) => println!("nop"),
                Err(e) => println!("t'as foiré: {}", e),
            }
        }
        Err(e) => println!("pareil: {}", e),
    }
}