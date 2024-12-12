// Remove these two lines if you're coding in this file
// #![allow(dead_code)]
// #![allow(unused_variables)]

// use std::path::Path;
use std::io::stdin;
use std::io::{self, Write};
use crate::maths::{_gen_prime, _gcd, _mod_inverse};


// CONSTANTS DECLARATION
const _KP_FOLDER: &str = "keypair/";
const _PUB_EXT: &str = "pub";
const _PRIV_EXT: &str = "priv";
const _SEED: u128 = 2;



// constants list for the SHA256 algorithm
const _K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b,
    0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01,
    0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7,
    0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152,
    0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
    0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
    0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819,
    0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08,
    0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f,
    0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];

const _A: u32 = 0x6a09e667;
const _B: u32 = 0xbb67ae85;
const _C: u32 = 0x3c6ef372;
const _D: u32 = 0xa54ff53a;
const _E: u32 = 0x510e527f;
const _F: u32 = 0x9b05688c;
const _G: u32 = 0x1f83d9ab;
const _H: u32 = 0x5be0cd19;

// A SHA256 Hash has always a size of 64 characters, according
// to the SHA256 algorithm.
type MyHash = [char; 64];

// Error type for the KeyPair struct.
pub enum KeyErr {
    _KeyPairsFolderNotFound,
    _KeyFileNotFound,
    _InvalidKeyFile,
    KeyAlreadyExists,
    _FileCreationError,
    _PrimeGenerationError,
}


struct RSAKey {
    pub key: String,
    _is_encrypted: bool,
}
pub struct KeyPair {
    private_key: RSAKey,
    public_key: RSAKey,
}
impl KeyPair {
    pub fn new() -> KeyPair {
        let pub_key = RSAKey {
            key: String::new(),
            _is_encrypted: false,
        };
        let priv_key = RSAKey {
            key: String::new(),
            _is_encrypted: false,
        };
        
        return KeyPair {
            private_key: priv_key,
            public_key: pub_key,
        };
    }

    // Generates a pair of RSA keys for the KeyPair.
    pub fn generate_keys(&mut self, crypt: bool) -> Result<(), KeyErr> {


        if !self.public_key.key.is_empty() || !self.private_key.key.is_empty() {
            return Err(KeyErr::KeyAlreadyExists);
        }

        let (pbk, pvk) = gen_rsa_keys(crypt)?;
        self.public_key = pbk;
        self.private_key = pvk;
        
        return Ok(());
    }

    // Returns the public and private keys as strings
    // (doesn't handle encryption for now)
    pub fn get_keys_str(&self) -> (String, String) {
        return (self.public_key.key.clone(), self.private_key.key.clone());
    }

    // fn save_to_file(&self) -> Result<(), KeyErr>
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
fn gen_rsa_keys(crypt: bool) -> Result<(RSAKey, RSAKey), KeyErr> {
    let _hash: Option<MyHash> = match crypt {
        false => None,
        // Asking for password etc
        true => {
            let mut pwd = String::new();

            print!("Enter the password for the key: ");
            io::stdout().flush().unwrap();

            // If we could use the rpassword crate we would,
            // but I think it's forbidden to use external crates..
            // Anyway, check this: https://crates.io/crates/rpassword

            stdin().read_line(&mut pwd).unwrap();
            Some(_str_to_hash(&pwd))

        }
    };

    let timer_start = std::time::Instant::now();
    print!("Generating RSA keys.. ");
    io::stdout().flush().unwrap();

    let p = _gen_prime();
    let q = _gen_prime();

    let (n, d, e) = _rsa_nde(p, q);


    let pub_key = RSAKey {
        key: format!("{}:{}", n, e),
        _is_encrypted: crypt,
    };

    let priv_key = RSAKey {
        key: format!("{}:{}", n, d),
        _is_encrypted: crypt,
    };

    let elapsed = timer_start.elapsed();
    println!("Done. ({}s)", elapsed.as_secs_f64());
    
    return Ok((pub_key, priv_key));

}





// I feel the AFIT nostalgia coming back to me
// According to the RSA algorithm up there, i think we need to
// find the n, d and e values from the p and q values.
fn _rsa_nde(p: u128, q: u128) -> (u128, u128, u128) {

    let n: u128 = p as u128 * q as u128;
    let z: u128 = (p - 1) * (q - 1);

    // choose a random e such that e is coprime with z
    // if a number is prime, it is coprime with all other numbers.
    // so we can choose a random prime number for e
    let mut e: u128 = _gen_prime();
    
    while _gcd(e, z) != 1 {
        e = _gen_prime();
    }
    
    let d: u128 = _mod_inverse(e, z).unwrap();

    return (n, d, e);
}





// You will find simple explanations about the SHA256 Hashing algorithm here: 
// https://www.simplilearn.com/tutorials/cyber-security-tutorial/
// sha-256-algorithm 
// Or even the Wikipedia Pseudo code:
// https://en.wikipedia.org/wiki/SHA-2#Pseudocode
// N.B: A SHA256 Hash has always a size of 64 characters,
// so we use a fixed size array of 64 characters to store it.

// Takes a string and returns its SHA256 hash.
fn _str_to_hash(_src: &str) -> MyHash {
    todo!();
}

