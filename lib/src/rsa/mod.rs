use super::sha256::sha256;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use libaes::Cipher;
use num_bigint::BigUint;
use num_primes::Generator;
use rand::Rng;
use sha1::Digest;
use sha1::Sha1;
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
    timestamp: u32,
    user_id: String,
}

struct _ASN1Key {
    value: String,
}

impl KeyPair {
    pub fn generate(bit_length: usize, user_id: &str) -> KeyPair {
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

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;

        return KeyPair {
            n: n,
            e: e,
            d: d,
            p: p,
            q: q,
            timestamp: timestamp,
            user_id: user_id.to_string(),
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

    pub fn pub_to_pem(&self) -> String {
        let pub_packet = self.publickeypacket();
        let formatted_uid = format_user_id(&self.user_id);
        let uid_packet = uidpacket(&formatted_uid);
        let signature_packet = self.signaturepacket(&formatted_uid);
        let mut full = vec![];
        full.extend(&pub_packet);
        full.extend(&uid_packet);
        full.extend(&signature_packet);
        let b64 = STANDARD.encode(&full);
        let crc = crc24(&full);
        let crcbytes = [(crc >> 16) as u8, (crc >> 8) as u8, crc as u8];
        let base64crc = STANDARD.encode(&crcbytes);
        let mut out = String::new();
        out.push_str("-----BEGIN PGP PUBLIC KEY BLOCK-----\n\n");
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

    pub fn priv_to_pem(&self, password: &str) -> String {
        let priv_packet = self.privatepacket(password);
        let formatted_uid = format_user_id(&self.user_id);
        let uid_packet = uidpacket(&formatted_uid);
        let sig_packet = self.signaturepacket(&formatted_uid);
        let mut full = Vec::new();
        full.extend(&priv_packet);
        full.extend(&uid_packet);
        full.extend(&sig_packet);
        let b64 = STANDARD.encode(&full);
        let crc = crc24(&full);
        let crc_bytes = [(crc >> 16) as u8, (crc >> 8) as u8, crc as u8];
        let crc_b64 = STANDARD.encode(&crc_bytes);
        let mut pem = String::new();
        pem.push_str("-----BEGIN PGP PRIVATE KEY BLOCK-----\n\n");
        for chunk in b64.as_bytes().chunks(64) {
            pem.push_str(&String::from_utf8_lossy(chunk));
            pem.push('\n');
        }
        pem.push('=');
        pem.push_str(&crc_b64);
        pem.push('\n');
        pem.push_str("-----END PGP PRIVATE KEY BLOCK-----\n");
        pem
    }

    // The next two function follow the following idea :
    /*
    find the tag , ex 0x05 for secret key)
    read the packet length
    skip metadata (version, timestamp, algorithm)
    parse through each MPI
    */

    // We have the same start for both,
    // we strip the pem format out of the file to keep only encoded data

    pub fn pub_from_pem(
        _pem: &str,
        user_id: &str,
    ) -> Result<KeyPair, RSAError> {
        let content: String = _pem
            .lines()
            .filter(|line| {
                !line.starts_with("-----")
                    && !line.starts_with("Version:")
                    && !line.trim().is_empty()
                    && !line.starts_with('=')
            })
            .collect::<Vec<_>>()
            .join("");
        let decoded = match STANDARD.decode(&content) {
            Ok(bytes) => bytes,
            Err(_) => return Err(RSAError::BadPEMFormat),
        };
        let mut i = 0;
        while i < decoded.len() {
            let tag = decoded[i] & 0x3F;
            i += 1;

            let (length, len_bytes) = if decoded[i] < 192 {
                (decoded[i] as usize, 1)
            } else if decoded[i] <= 223 {
                let b1 = decoded[i] as usize;
                let b2 = decoded[i + 1] as usize;
                (((b1 - 192) << 8) + b2 + 192, 2)
            } else if decoded[i] == 255 {
                (
                    ((decoded[i + 1] as usize) << 24)
                        | ((decoded[i + 2] as usize) << 16)
                        | ((decoded[i + 3] as usize) << 8)
                        | (decoded[i + 4] as usize),
                    5,
                )
            } else {
                return Err(RSAError::BadPEMFormat);
            };
            i += len_bytes;
            if tag == 6 {
                i += 1;
                let timestamp =
                    u32::from_be_bytes(decoded[i..i + 4].try_into().unwrap());
                i += 4;
                let _algo = decoded[i];
                i += 1;
                let n = parse_mpi(&decoded, &mut i);
                let e = parse_mpi(&decoded, &mut i);
                return Ok(KeyPair {
                    n,
                    e,
                    d: BigUint::default(),
                    p: BigUint::default(),
                    q: BigUint::default(),
                    timestamp,
                    user_id: user_id.to_string(),
                });
            } else {
                i += length;
            }
        }
        Err(RSAError::BadPEMFormat)
    }

    pub fn priv_from_pem(
        pem: &str,
        password: &str,
        user_id: &str,
    ) -> Result<KeyPair, RSAError> {
        let content: String = pem
            .lines()
            .filter(|line| {
                !line.starts_with("-----")
                    && !line.starts_with("Version:")
                    && !line.trim().is_empty()
                    && !line.starts_with('=')
            })
            .collect::<Vec<_>>()
            .join("");
        let decoded = match STANDARD.decode(&content) {
            Ok(bytes) => bytes,
            Err(_) => return Err(RSAError::BadPEMFormat),
        };
        let mut i = 0;
        while i < decoded.len() {
            let tag = decoded[i];
            let tag_type = tag & 0x3F;
            i += 1;
            let (len, len_size) = if decoded[i] < 192 {
                (decoded[i] as usize, 1)
            } else if decoded[i] <= 223 {
                let b1 = decoded[i] as usize;
                let b2 = decoded[i + 1] as usize;
                (((b1 - 192) << 8) + b2 + 192, 2)
            } else if decoded[i] == 255 {
                (
                    ((decoded[i + 1] as usize) << 24)
                        | ((decoded[i + 2] as usize) << 16)
                        | ((decoded[i + 3] as usize) << 8)
                        | (decoded[i + 4] as usize),
                    5,
                )
            } else {
                return Err(RSAError::BadPEMFormat);
            };
            i += len_size;
            if tag_type == 5 {
                let start = i;
                i += 1;
                let timestamp = u32::from_be_bytes([
                    decoded[i],
                    decoded[i + 1],
                    decoded[i + 2],
                    decoded[i + 3],
                ]);
                i += 4;
                i += 1;
                let n = parse_mpi(&decoded, &mut i);
                let e = parse_mpi(&decoded, &mut i);
                let s2k_tag = decoded[i];
                i += 1;
                assert_eq!(s2k_tag, 0xFE);
                let _s2k_type = decoded[i];
                i += 1;
                let _hash_algo = decoded[i];
                i += 1;
                let _cipher_algo = decoded[i];
                i += 1;
                let salt = &decoded[i..i + 8];
                i += 8;
                let count_byte = decoded[i];
                i += 1;
                let iv = &decoded[i..i + 16];
                i += 16;
                let encrypted = &decoded[i..start + len];
                let iter_count = 16usize
                    .wrapping_add((count_byte & 15) as usize)
                    .wrapping_shl(((count_byte >> 4) + 6) as u32);
                let mut s2k_input = Vec::new();
                s2k_input.extend(salt);
                s2k_input.extend(password.as_bytes());
                let repeated = s2k_input
                    .repeat((iter_count as usize / s2k_input.len()) + 1);
                let key =
                    &sha256(repeated[..iter_count as usize].to_vec())[0..16];
                let key_array: &[u8; 16] = key.try_into().unwrap();

                let cipher = Cipher::new_128(key_array);
                let decrypted =
                    cipher.cfb128_decrypt(iv.try_into().unwrap(), encrypted);

                let mut j = 0;
                let d = parse_mpi(&decrypted, &mut j);
                let p = parse_mpi(&decrypted, &mut j);
                let q = parse_mpi(&decrypted, &mut j);
                let _u = parse_mpi(&decrypted, &mut j);
                return Ok(KeyPair {
                    n,
                    e,
                    d,
                    p,
                    q,
                    timestamp,
                    user_id: user_id.to_string(),
                });
            }
            i += len;
        }
        Err(RSAError::BadPEMFormat)
    }

    // Generates the public packet of the openpgp tag 6
    // a tag is the integer that represents the kind of data you treat
    // for example public-key-packet is 6, secret-key-packet is 5.. etc

    // so this one generates the public packet that contains :
    // version, timestamp, id of the rsa algorithm,
    // the mpi encoded n and e and then it adds the openpgp header

    pub fn publickeypacket(&self) -> Vec<u8> {
        let body = self.publickey_body();
        let mut packet = vec![0xc0 | 6];
        packet.extend(encode_length(body.len()));
        packet.extend(&body);
        packet
    }

    pub fn publickey_body(&self) -> Vec<u8> {
        let mut body = Vec::new();
        body.push(0x04);
        body.extend_from_slice(&self.timestamp.to_be_bytes());
        body.push(0x01);
        body.extend(encode_n_e(&self.n));
        body.extend(encode_n_e(&self.e));
        body
    }

    pub fn privatepacket(&self, password: &str) -> Vec<u8> {
        let u = self.q.modinv(&self.p).unwrap();
        let mut mpis = Vec::new();
        mpis.extend(encode_n_e(&self.d));
        mpis.extend(encode_n_e(&self.p));
        mpis.extend(encode_n_e(&self.q));
        mpis.extend(encode_n_e(&u));
        let mut hasher = Sha1::new();
        hasher.update(&mpis);
        let hash = hasher.finalize();
        mpis.extend(&hash);
        let mut rng = rand::rng();
        let salt: [u8; 8] = rng.random();
        let count_byte = 0x60;
        let iter_count = (16 + (count_byte & 15)) << ((count_byte >> 4) + 6);
        let mut s2k_input = Vec::new();
        s2k_input.extend(&salt);
        s2k_input.extend(password.as_bytes());
        let repeated_input =
            s2k_input.repeat((iter_count / s2k_input.len()) + 1);
        let digest = sha256(repeated_input[..iter_count].to_vec());
        let key = &digest[..16];
        let key_array: &[u8; 16] =
            key.try_into().expect("Key must be 16 bytes");
        let iv: [u8; 16] = rng.random();
        let cipher = Cipher::new_128(key_array);
        let encrypted = cipher.cfb128_encrypt(&iv, &mpis);
        let mut body = Vec::new();
        body.push(0x04);
        body.extend_from_slice(&self.timestamp.to_be_bytes());
        body.push(0x01);
        body.extend(encode_n_e(&self.n));
        body.extend(encode_n_e(&self.e));
        body.push(0xFE);
        body.push(0x07);
        body.push(0x03);
        body.push(0x08);
        body.extend(&salt);
        body.push(count_byte as u8);
        body.extend(&iv);
        body.extend(&encrypted);
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

    pub fn signaturepacket(&self, raw_user_id: &str) -> Vec<u8> {
        fn encode_subpacket(subpacket_type: u8, content: &[u8]) -> Vec<u8> {
            let mut packet = Vec::new();
            let length = content.len() + 1;
            if length < 192 {
                packet.push(length as u8);
            } else if length <= 8383 {
                let len = length - 192;
                packet.push(((len >> 8) + 192) as u8);
                packet.push((len & 0xff) as u8);
            } else {
                packet.push(255);
                packet.extend(&(length as u32).to_be_bytes());
            }
            packet.push(subpacket_type);
            packet.extend_from_slice(content);
            packet
        }
        let timestamp = self.timestamp;
        let creation_sub = encode_subpacket(2, &timestamp.to_be_bytes());
        let pubkey_body = self.publickey_body();
        let mut pk_prefix = vec![0x99];
        pk_prefix.extend(&(pubkey_body.len() as u16).to_be_bytes());
        pk_prefix.extend(&pubkey_body);
        let keyid_hash = Sha1::digest(&pk_prefix);
        let short_keyid = &keyid_hash[12..];
        let issuer_sub = encode_subpacket(16, short_keyid);
        let mut hashed_subpackets = Vec::new();
        hashed_subpackets.extend_from_slice(&creation_sub);
        hashed_subpackets.extend_from_slice(&issuer_sub);
        let unhashed_subpackets: Vec<u8> = Vec::new();
        let mut hashed_data = vec![0x04, 0x13, 0x01, 0x08];
        hashed_data.extend(&(hashed_subpackets.len() as u16).to_be_bytes());
        hashed_data.extend(&hashed_subpackets);
        let mut to_hash = Vec::new();
        to_hash.extend(&pk_prefix);
        let uid_body = raw_user_id.as_bytes();
        to_hash.push(0xB4);
        to_hash.extend(&(uid_body.len() as u32).to_be_bytes());
        to_hash.extend(uid_body);
        to_hash.extend(&hashed_data);
        to_hash.push(0x04);
        to_hash.push(0xFF);
        to_hash.extend(&(hashed_data.len() as u32).to_be_bytes());
        let hash = sha256(to_hash);
        assert_eq!(hash.len(), 32);
        let hash_prefix = &hash[0..2];
        let mut digest_info = vec![];
        digest_info.extend_from_slice(&[
            0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65,
            0x03, 0x04, 0x02, 0x01, 0x05, 0x00, 0x04, 0x20,
        ]);
        digest_info.extend_from_slice(&hash);
        assert_eq!(digest_info.len(), 51);
        let modulus_len = (self.n.bits() + 7) / 8;
        let ps_len = modulus_len as usize - 3 - digest_info.len();
        let mut padded = Vec::with_capacity(modulus_len as usize);
        padded.push(0x00);
        padded.push(0x01);
        padded.extend(std::iter::repeat(0xFF).take(ps_len));
        padded.push(0x00);
        padded.extend(digest_info);
        assert_eq!(padded.len(), modulus_len as usize);
        let m = BigUint::from_bytes_be(&padded);
        let sig = m.modpow(&self.d, &self.n);
        let mpi = encode_n_e(&sig);
        let mut final_body = hashed_data;
        final_body.extend(&(unhashed_subpackets.len() as u16).to_be_bytes());
        final_body.extend(&unhashed_subpackets);
        final_body.extend(hash_prefix);
        final_body.extend(mpi);
        let mut packet = vec![0xc0 | 2];
        packet.extend(encode_length(final_body.len()));
        packet.extend(final_body);
        packet
    }
}

// This one is used in from pem functions

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
    let uid_bytes = user_id.as_bytes();
    let mut packet = vec![0xc0 | 13];
    packet.extend(encode_length(uid_bytes.len()));
    packet.extend(uid_bytes);
    packet
}

fn format_user_id(raw_uid: &str) -> String {
    if raw_uid.contains('@') {
        format!("<{}>", raw_uid)
    } else {
        format!("{} <{}@epita.fr>", raw_uid, raw_uid)
    }
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

fn parse_mpi(data: &[u8], i: &mut usize) -> BigUint {
    let bit_len = ((data[*i] as u16) << 8) | data[*i + 1] as u16;
    let byte_len = ((bit_len + 7) / 8) as usize;
    *i += 2;
    let value = BigUint::from_bytes_be(&data[*i..*i + byte_len]);
    *i += byte_len;
    value
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
