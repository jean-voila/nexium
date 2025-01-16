// Remove these two lines if you're coding in this file
// #![allow(dead_code)]
// #![allow(unused_variables)]

// use std::path::Path;
// use std::io::stdin;
use crate::maths::{_gcd, _gen_prime, _mod_inverse};
use std::io::{self, Write};

// CONSTANTS DECLARATION
const _KP_FOLDER: &str = "keypair/";

struct PubRSAKey {
    n: u128,
    e: u128,
}

struct PrivRSAKey {
    n: u128,
    d: u128,
}

pub struct KeyPair {
    private_key: PrivRSAKey,
    public_key: PubRSAKey,
}

// Temporary debug function
// Don't forget to remove it when we've implemented
// the crypting functions.
impl KeyPair {
    pub fn to_strings(&self) -> (String, String) {
        return (
            format!("{}:{}", self.public_key.n, self.public_key.e),
            format!("{}:{}", self.private_key.n, self.private_key.d),
        );
    }
}

impl KeyPair {
    pub fn new() -> KeyPair {
        let (pbk, prk) = gen_rsa_keys().unwrap();

        return KeyPair {
            private_key: prk,
            public_key: pbk,
        };
    }

    fn _to_file(&self) -> Option<()> {
        // We'll use the PEM format to store the keys.
        // https://stackoverflow.com/questions/27568570/
        // how-to-convert-raw-modulus-exponent-to-rsa-public-key-pem-format

        // Docs about the PEM format:
        // https://docs.fileformat.com/fr/web/pem/
        // #:~:text=R%C3%A9f%C3%A9rences-,
        // Qu'est%2Dce%20qu'un%20fichier%20PEM%20%3F,
        // une%20combinaison%20d'autres%20certificats.

        todo!();
    }
    // fn from_file(path_p: &Path) -> Result<Self, KeyErr>
    // fn wipe_keys(&mut self) -> Result<(), KeyErr>
}

// RSA Key Generation
// https://www.simplilearn.com/tutorials/cryptography-tutorial/rsa-algorithm
// Also check this out: https://en.wikipedia.org/wiki/RSA_(cryptosystem)
// You need to generate public and private keys before running the functions to
// generate your ciphertext and plaintext. They use certain variables and
// parameters, all of which are explained below:
//   (1) Choose two large prime numbers (p and q)
//   (2) Calculate n = p*q and z = (p-1)(q-1)
//   (3) Choose a number e where 1 < e < z
//   (4) Calculate d = e-1mod(p-1)(q-1)
//   (5) You can bundle private key pair as (n,d)
//   (6) You can bundle public key pair as (n,e)

// Generates a pair of RSA keys.
// crypt: If true, the keys will be encrypted.
// Returns a tuple containing the public and private keys.
fn gen_rsa_keys() -> Option<(PubRSAKey, PrivRSAKey)> {
    let timer_start = std::time::Instant::now();

    print!("Generating RSA keys.. ");
    io::stdout().flush().unwrap();

    let p = _gen_prime();
    let q = _gen_prime();

    let (n, d, e) = _rsa_nde(p, q);

    let elapsed = timer_start.elapsed();
    println!("Done. ({}s)", elapsed.as_secs_f64());

    return Some((PubRSAKey { n, e }, PrivRSAKey { n, d }));
}

// I feel the AFIT nostalgia coming back to me
// According to the RSA algorithm up there, i think we need to
// find the n, d and e values from the p and q values.

fn _rsa_nde(p: u128, q: u128) -> (u128, u128, u128) {
    let n: u128 = p * q;
    let z: u128 = (p - 1) * (q - 1);

    // choose a random e such that e is coprime with z
    let mut e: u128 = 2;
    while e < z {
        if _gcd(e, z) == 1 {
            break;
        }
        e += 1;
    }

    let d: u128 = _mod_inverse(e, z).unwrap();

    return (n, d, e);
}

