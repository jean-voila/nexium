use std::io::{Read, Write};

use num_bigint::BigUint;
use num_primes::Generator;

#[derive(Debug)]
pub enum RSAError {
    MessageTooBig,
    EmptyMessage,
    BadSignatureFormat,
    FileWriteError,
    FileReadError,
    BadPEMFormat,
}

pub enum PEMType {
    PublicKey,
    PrivateKey,
}

pub struct KeyPair {
    pub bit_length: usize,
    n: BigUint,
    e: BigUint,
    d: BigUint,
    p: BigUint,
    q: BigUint,
}

struct _ASN1Key {
    value: String,
}

impl KeyPair {
    pub fn generate(bit_length: usize) -> KeyPair {
        // I've only found this ugly way to convert the BigUint... sorry
        let p_prime: num_primes::BigUint = Generator::new_prime(bit_length / 2);
        let p: BigUint =
            BigUint::parse_bytes(p_prime.to_string().as_bytes(), 10).unwrap();

        let q_prime: num_primes::BigUint = Generator::new_prime(bit_length / 2);
        let q: BigUint =
            BigUint::parse_bytes(q_prime.to_string().as_bytes(), 10).unwrap();

        // N.B : We're not wondering about the performance cost of using
        // the clone() method here, because the user will generate the key
        // only once(theorically).
        let n: BigUint = p.clone() * q.clone();
        let phi: BigUint = (p.clone() - 1u32) * (q.clone() - 1u32);

        let e: BigUint = BigUint::from(65537u32);

        let d: BigUint = e.clone().modinv(&phi).unwrap();

        return KeyPair {
            bit_length,
            n: n,
            e: e,
            d: d,
            p: p,
            q: q,
        };
    }

    ///RSA Signing: S = M^d mod n
    pub fn sign(&self, message: &[u8]) -> Result<BigUint, RSAError> {
        if message.is_empty() {
            return Err(RSAError::EmptyMessage);
        }

        let m = BigUint::from_bytes_be(message);

        if &m >= &self.n {
            return Err(RSAError::MessageTooBig);
        }

        Ok(m.modpow(&self.d, &self.n))
    }

    ///RSA  Verification: M' = S^e mod n
    pub fn check_signature(
        &self,
        message: &[u8],
        signature: &BigUint,
    ) -> Result<bool, RSAError> {
        if message.is_empty() {
            return Err(RSAError::EmptyMessage);
        }
        let m_verif = signature.modpow(&self.e, &self.n);
        let message_b = BigUint::from_bytes_be(message);

        Ok(m_verif == message_b)
    }

    pub fn crypt(&self, message: &[u8]) -> Result<BigUint, RSAError> {
        if message.is_empty() {
            return Err(RSAError::EmptyMessage);
        }

        let m = BigUint::from_bytes_be(message);

        if &m >= &self.n {
            return Err(RSAError::MessageTooBig);
        }

        Ok(m.modpow(&self.e, &self.n))
    }
    pub fn decrypt(&self, message: &[u8]) -> Result<BigUint, RSAError> {
        if message.is_empty() {
            return Err(RSAError::EmptyMessage);
        }

        let m = BigUint::from_bytes_be(message);

        if &m >= &self.n {
            return Err(RSAError::MessageTooBig);
        }

        Ok(m.modpow(&self.d, &self.n))
    }

    pub fn to_pem(&self) -> (String, String) {
        todo!();
    }

    fn from_pem(_pem: &str) -> Result<KeyPair, RSAError> {
        todo!();
    }

    pub fn to_file(&self, path: &str) -> Result<(), RSAError> {
        let file = std::fs::File::create(path);
        match file {
            Ok(mut f) => {
                let to_write = self.to_pem().1;

                // Writing the PEM file
                match f.write_all(to_write.as_bytes()) {
                    Ok(_) => (),
                    Err(_) => {
                        return Err(RSAError::FileWriteError);
                    }
                }

                Ok(())
            }
            Err(_) => Err(RSAError::FileWriteError),
        }
    }

    pub fn from_file(path: &str) -> Result<KeyPair, RSAError> {
        let file = std::fs::File::open(path);
        match file {
            Ok(mut f) => {
                let mut contents = String::new();
                match f.read_to_string(&mut contents) {
                    Ok(_) => {
                        // Parsing the PEM file
                        match KeyPair::from_pem(&contents) {
                            Ok(key_pair) => Ok(key_pair),
                            Err(_) => Err(RSAError::BadPEMFormat),
                        }
                    }
                    Err(_) => Err(RSAError::FileReadError),
                }
            }
            Err(_) => Err(RSAError::FileReadError),
        }
    }
}

impl std::fmt::Debug for KeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "n: {:x};\ne: {:x};\nd: {:x};\np: {:x};\nq: {:x}\n",
            self.n, self.e, self.d, self.p, self.q
        )
    }
}
