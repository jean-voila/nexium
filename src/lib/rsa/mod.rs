use super::sha256::sha256;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use core::hash;
use num_bigint::BigUint;
use num_primes::Generator;
use std::time::{SystemTime, UNIX_EPOCH};

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
            n: n,
            e: e,
            d: d,
            p: p,
            q: q,
        };
    }

    pub fn sign(&self, message: Vec<u8>) -> Result<BigUint, RSAError> {
        if message.is_empty() {
            return Err(RSAError::EmptyMessage);
        }

        let hash = sha256(message);
        let m = BigUint::from_bytes_be(&hash);

        if &m >= &self.n {
            return Err(RSAError::MessageTooBig);
        }

        Ok(m.modpow(&self.d, &self.n))
    }
    pub fn check_signature(
        &self,
        message: Vec<u8>,
        signature: &BigUint,
    ) -> Result<bool, RSAError> {
        if message.is_empty() {
            return Err(RSAError::EmptyMessage);
        }
        let hash_verif = sha256(message);
        let m = BigUint::from_bytes_be(&hash_verif);

        let decrypted_signature = signature.modpow(&self.e, &self.n);

        Ok(decrypted_signature == m)
    }

    pub fn crypt(&self, message: Vec<u8>) -> Result<BigUint, RSAError> {
        if message.is_empty() {
            return Err(RSAError::EmptyMessage);
        }

        let m = BigUint::from_bytes_be(message.as_slice());

        if &m >= &self.n {
            return Err(RSAError::MessageTooBig);
        }

        Ok(m.modpow(&self.e, &self.n))
    }
    pub fn decrypt(&self, message: Vec<u8>) -> Result<BigUint, RSAError> {
        if message.is_empty() {
            return Err(RSAError::EmptyMessage);
        }

        let m = BigUint::from_bytes_be(message.as_slice());

        if &m >= &self.n {
            return Err(RSAError::MessageTooBig);
        }

        Ok(m.modpow(&self.d, &self.n))
    }

    // Concatenate packet of the public key with packet of user id
    // does the crc24
    // encodes everything in base64
    // puts it in format with header of gpg (ASCII ARMOR)

    //Like this :
    /* I took out the version because i still don't know if we should put it early on
    -----BEGIN PGP PUBLIC KEY BLOCK-----
    ...
    =CRC
    -----END PGP PUBLIC KEY BLOCK-----
    */

    pub fn pub_to_pem(&self, user_id: &str) -> String {
        let public_packet = self.publickeypacket();
        let uid_packet = uidpacket(user_id);
        let mut signed_data = Vec::new();
        signed_data.extend(&public_packet);
        signed_data.extend(&uid_packet);
        let signature_packet = self.signature_packet(&signed_data);

        let mut full = vec![];
        full.extend(public_packet);
        full.extend(uid_packet);
        full.extend(signature_packet);

        // let b64 = base64::encode(&full);
        let b64 = STANDARD.encode(&full);
        let crc = crc24(&full);
        let crcbytes = [(crc >> 16) as u8, (crc >> 8) as u8, crc as u8];

        // let base64crc = base64::encode(&crcbytes);
        let base64crc = STANDARD.encode(&crcbytes);
        let mut out = String::new();
        out.push_str("-----BEGIN PGP PUBLIC KEY BLOCK-----\n\n");

        // I did not put version yet because i don't know if we need to write it here
        // or if the push after is enough

        for chunk in b64.as_bytes().chunks(64) {
            out.push_str(&String::from_utf8_lossy(chunk));
            out.push('\n');
        }
        out.push('=');
        out.push_str(&base64crc);
        out.push('\n');
        out.push_str("-----END PGP PUBLIC KEY BLOCK-----\n");
        out
    }
    pub fn priv_to_pem(&self) -> String {
        let secret_packet = self.privatekeypacket();

        // let b64 = base64::encode(&secret_packet);
        let b64 = STANDARD.encode(&secret_packet);
        let crc = crc24(&secret_packet);
        let crcbytes = [(crc >> 16) as u8, (crc >> 8) as u8, crc as u8];

        // let base64crc = base64::encode(&crcbytes);
        let base64crc = STANDARD.encode(&crcbytes);
        let mut out = String::new();
        out.push_str("-----BEGIN PGP PRIVATE KEY BLOCK-----\n");
        for chunk in b64.as_bytes().chunks(64) {
            out.push_str(&String::from_utf8_lossy(chunk));
            out.push('\n');
        }
        out.push('=');
        out.push_str(&base64crc);
        out.push('\n');
        out.push_str("-----END PGP PRIVATE KEY BLOCK-----");
        out
    }
    pub fn priv_from_pem(_pem: &str) -> Result<KeyPair, RSAError> {
        todo!();
    }
    pub fn pub_from_pem(_pem: &str) -> Result<KeyPair, RSAError> {
        todo!();
    }
    // Generates the public packet of the openpgp tag 6
    // a tag is the integer that represents the kind of data you treat
    // for example public-key-packet is 6, secret-key-packet is 5.. etc

    // so this one generates the public packet that contains :
    // version, timestamp, id of the rsa algorithm,
    // the mpi encoded n and e and then it adds the openpgp header

    fn publickeypacket(&self) -> Vec<u8> {
        let mut body = Vec::new();
        //push version
        body.push(0x04);

        // seconds since 01/01/1970 as jean explained to us
        // general term that add in
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;
        body.extend_from_slice(&timestamp.to_be_bytes());
        //we add rsa algo id below
        body.push(0x01);

        // mpi encoded n and e
        body.extend(encode_n_e(&self.n));
        body.extend(encode_n_e(&self.e));

        let mut packet = vec![0xc0 | 6];
        packet.extend(encode_length(body.len()));
        packet.extend(body);
        packet
    }
    fn privatekeypacket(&self) -> Vec<u8> {
        let mut body = Vec::new();

        //version, same idea as in public
        body.push(0x04);

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;
        body.extend_from_slice(&timestamp.to_be_bytes());

        // Add of tthe public key of rsa algo
        body.push(0x01);

        // Here we push the public encoded n and e
        body.extend(encode_n_e(&self.n));
        body.extend(encode_n_e(&self.e));

        // 0x00 = indicates that the key is not encrypted yet
        body.push(0x00);

        // Here we add the secrets encodings of n and e
        body.extend(encode_n_e(&self.d));
        body.extend(encode_n_e(&self.p));
        body.extend(encode_n_e(&self.q));
        let u = self.q.modinv(&self.p).unwrap();
        body.extend(encode_n_e(&u));

        // push the header
        let mut packet = vec![0xc0 | 5];
        packet.extend(encode_length(body.len()));
        packet.extend(body);

        packet
    }
    // this function returns the signature packet,
    // it's a packet that we will use in the publickey to pem
    // and it is a "certification signature" over the public key packet
    // and the uid packet
    // It is a direct link between the user that corresponds to this uid
    // ,the signature and the public key

    // Contains the metadata : version, type, algorithm, hash (in order)
    // that we push at the beginning, type and hash are the one we use
    // so respectively 0x13 -> RSA and 0x08 -> sha256
    // Also contains prefixes (empty and first 2 bytes hash prefix)
    // At last, contains the body (mpi encoded)

    pub fn signature_packet(&self, signed_data: &[u8]) -> Vec<u8> {
        let hash = sha256(signed_data.to_vec());
        let m = BigUint::from_bytes_be(&hash);
        let signature = m.modpow(&self.d, &self.n);
        let mpi = encode_n_e(&signature);
        let mut body = Vec::new();
        body.push(0x04);
        body.push(0x13);
        body.push(0x01);
        body.push(0x08);
        body.extend_from_slice(&[0x00, 0x00]);
        body.extend_from_slice(&hash[..2]);
        body.extend(mpi);
        let mut packet = vec![0xc0 | 2]; // Packet tag 2 = signature
        packet.extend(encode_length(body.len()));
        packet.extend(body);
        packet
    }
}

// this function encodes a biguint to the mpi format
//(multi-precision-integer) used in openpgp
// -2 bytes for length
// -bytes of the integer in bigendian

fn encode_n_e(value: &BigUint) -> Vec<u8> {
    let bytes = value.to_bytes_be();
    let bit_len = (bytes.len() * 8 - bytes[0].leading_zeros() as usize) as u16;
    let mut result = Vec::new();
    result.extend_from_slice(&bit_len.to_be_bytes());
    result.extend(bytes);
    result
}

// this one encodes the length of an openpgp packet
// following this format : (RFC 4880)
// 3 different cases as we can see below
// I believe the function is kinda self-explaining
// but just in case, depending on the len of the packet
// we encode it on 1 byte, 2 bytes or 0xff + 4 bytes

fn encode_length(len: usize) -> Vec<u8> {
    if len < 192 {
        vec![len as u8]
    } else if len <= 8383 {
        let len = len - 192;
        vec![((len >> 8) + 192) as u8, (len & 0xff) as u8]
    } else {
        let mut v = vec![0xff];
        v.extend_from_slice(&(len as u32).to_be_bytes());
        v
    }
}

// function that treats the user id (we have a function that gets it)
// so I just put it in parameter, this way it treats it right away
// by putting it UTF-8, after a header that contains (tag+length)
// and when we have uid we have email so it's 2 in 1

fn uidpacket(user_id: &str) -> Vec<u8> {
    let email = format!("{}@epita.fr", user_id);
    let uid = email;
    let bytesuid = uid.as_bytes().to_vec();
    let mut packet = vec![0xc0 | 13];
    packet.extend(encode_length(bytesuid.len()));
    packet.extend(bytesuid);
    packet
}

// before base64 encoding, "cyclic redundancy check"
// we check the integrity of the corresponding information
// ex: if we copy a key from a mail and a character changes,
// the crc check will not pass and then not work

// theory :
// Initialized at : 0xB704CE
// use the polynom : 0x1864CFB
// return a checksum on 3 bytes
// later encoded in base64

fn crc24(data: &[u8]) -> u32 {
    let mut crc = 0xB704CEu32;
    for b in data {
        crc ^= (*b as u32) << 16;
        for _ in 0..8 {
            crc <<= 1;
            if (crc & 0x1000000) != 0 {
                crc ^= 0x1864CFB;
            }
        }
    }
    crc & 0xFFFFFF
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
