use base64;
use num_bigint::BigUint;
use num_primes::Generator;
use std::time::{SystemTime, UNIX_EPOCH};

// See the PKCS#1 standard for more information

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

    // this function encodes a biguint to the mpi format
    //(multi-precision-integer) used in openpgp
    // -2 bytes for length
    // -bytes of the integer in bigendian

    fn encode_n_e(value: &BigUint) -> Vec<u8> {
        let bytes = value.to_bytes_be();
        let bit_len =
            (bytes.len() * 8 - bytes[0].leading_zeros() as usize) as u16;
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

    // Generates the public packet of the openpgp tag 6
    // a tag is the integer that represents the kind of data you treat
    // for example public-key-packet is 6, secret-key-packet is 5.. etc

    // so this one generates the public packet that contains :
    // version, timestamp, id of the rsa algorithm,
    // the mpi encoded n and e and then it adds the openpgp header

    fn publickeypacket(key: &KeyPair) -> Vec<u8> {
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
        body.extend(Self::encode_n_e(&key.n));
        body.extend(Self::encode_n_e(&key.e));

        let mut packet = vec![0xc0 | 6];
        packet.extend(Self::encode_length(body.len()));
        packet.extend(body);
        packet
    }

    // same idea as in public, I've put notes inside code on what is happening

    fn privatekeypacket(key: &KeyPair) -> Vec<u8> {
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
        body.extend(Self::encode_n_e(&key.n));
        body.extend(Self::encode_n_e(&key.e));

        // 0x00 = indicates that the key is not encrypted yet
        body.push(0x00);

        // Here we add the secrets encodings of n and e
        body.extend(Self::encode_n_e(&key.d));
        body.extend(Self::encode_n_e(&key.p));
        body.extend(Self::encode_n_e(&key.q));
        let u = key.q.modinv(&key.p).unwrap();
        body.extend(Self::encode_n_e(&u));

        // push the header
        let mut packet = vec![0xc0 | 5];
        packet.extend(Self::encode_length(body.len()));
        packet.extend(body);

        packet
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
        packet.extend(Self::encode_length(bytesuid.len()));
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

    // Concatenate packet of the public key with packet of user id
    // does the crc24
    // encodes everything in base64
    // puts it in format with header of gpg

    //Like this :
    /* I took out the version because i still don't know if we should put it early on
    -----BEGIN PGP PUBLIC KEY BLOCK-----
    ...
    =CRC
    -----END PGP PUBLIC KEY BLOCK-----
    */

    fn publickeygpg(key: &KeyPair, user_id: &str) -> String {
        let public_packet = Self::publickeypacket(key);
        let uid_packet = Self::uidpacket(user_id);
        let mut full = vec![];
        full.extend(public_packet);
        full.extend(uid_packet);

        // HERE WE NEED TO ADD SIGNATURE PACKET (waiting for antonin to do it)

        let b64 = base64::encode(&full);
        let crc = Self::crc24(&full);
        let crcbytes = [(crc >> 16) as u8, (crc >> 8) as u8, crc as u8];
        let base64crc = base64::encode(&crcbytes);
        let mut out = String::new();
        out.push_str("-----BEGIN PGP PUBLIC KEY BLOCK-----\n");

        // I did not put version yet because i don't know if we need to write it here
        // or if the push after is enough

        for chunk in b64.as_bytes().chunks(64) {
            out.push_str(&String::from_utf8_lossy(chunk));
            out.push('\n');
        }
        out.push('=');
        out.push_str(&base64crc);
        out.push('\n');
        out.push_str("-----END PGP PUBLIC KEY BLOCK-----");
        out
    }

    // same idea as the public one but this time we concatenate the packet
    // of the secret key
    // and we adapt the header

    // waiting for signature

    fn privatekeygpg(key: &KeyPair) -> String {
        let secret_packet = Self::privatekeypacket(key);
        let b64 = base64::encode(&secret_packet);
        let crc = Self::crc24(&secret_packet);
        let crcbytes = [(crc >> 16) as u8, (crc >> 8) as u8, crc as u8];
        let base64crc = base64::encode(&crcbytes);
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

fn rsa_encrypt(key: &KeyPair, message: &BigUint) -> BigUint {
    return message.modpow(&key.e, &key.n);
}

fn rsa_decrypt(key: &KeyPair, message: &BigUint) -> BigUint {
    return message.modpow(&key.d, &key.n);
}
