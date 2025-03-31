use base64;
use num_bigint::BigUint;
use num_primes::Generator;

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

    //NEED TO COMMENT TO EXPLAIN FUNCTIONS HERE WILL DO IT

    fn encode_integer(n: &BigUint) -> Vec<u8> {
        let mut be = n.to_bytes_be();
        if be.first().map_or(false, |b| b & 0x80 != 0) {
            be.insert(0, 0x00);
        }
        let mut out = vec![0x02];
        out.extend(Self::encode_length(be.len()));
        out.extend(be);
        out
    }

    fn encode_length(len: usize) -> Vec<u8> {
        if len < 0x80 {
            vec![len as u8]
        } else {
            let lenb = len
                .to_be_bytes()
                .into_iter()
                .skip_while(|b| *b == 0)
                .collect::<Vec<_>>();
            let mut out = vec![0x80 | (lenb.len() as u8)];
            out.extend(lenb);
            out
        }
    }

    fn encode_sequence(elt: Vec<u8>) -> Vec<u8> {
        let mut out = vec![0x30];
        out.extend(Self::encode_length(elt.len()));
        out.extend(elt);
        out
    }

    pub fn public_key_formatted(&self) -> String {
        let n_encoded = Self::encode_integer(&self.n);
        let e_encoded = Self::encode_integer(&self.e);
        let public_key_sequence =
            Self::encode_sequence([n_encoded, e_encoded].concat());

        //Here we put the public key as a vec and we put the tag at the beginning
        let mut bitstr = vec![0x03];
        bitstr.extend(Self::encode_length(public_key_sequence.len() + 1));
        bitstr.push(0x00);
        bitstr.extend(public_key_sequence);

        //Here is the algorithmidentifier for rsa
        let rsa_oid = vec![
            0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01,
        ];
        let null_param = vec![0x05, 0x00];
        let algoident = Self::encode_sequence([rsa_oid, null_param].concat());

        // Here we combine the algorithmidentifier and the bitstring into
        // the "publickeyinfo" sequence
        let public_key_info =
            Self::encode_sequence([algoident, bitstr].concat());

        // We encode the DER-encoded public key as base64
        let pem_body = base64::encode(&public_key_info);

        // Here we apply the format of the PEM with appropriate headers and endings
        // we add line breaks every 64 characters for readability
        let pem = format!(
            "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----",
            pem_body
                .chars()
                .collect::<Vec<_>>()
                .chunks(64)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        );
        pem
    }

    //COMMENTS TO BE DONE BELOW
    // i'll write them under every step when I write my part of the report
    // so i'll think of a way to explain it clearly

    fn private_key_formatted(&self) -> _ASN1Key {
        let version = BigUint::from(0u8);
        let exponent1 = &self.d % (&self.p - 1u32);
        let exponent2 = &self.d % (&self.q - 1u32);
        let coefficient = self.q.modinv(&self.p).unwrap();
        let mut encoded = Vec::new();
        encoded.extend(Self::encode_integer(&version));
        encoded.extend(Self::encode_integer(&self.n));
        encoded.extend(Self::encode_integer(&self.e));
        encoded.extend(Self::encode_integer(&self.d));
        encoded.extend(Self::encode_integer(&self.p));
        encoded.extend(Self::encode_integer(&self.q));
        encoded.extend(Self::encode_integer(&exponent1));
        encoded.extend(Self::encode_integer(&exponent2));
        encoded.extend(Self::encode_integer(&coefficient));
        let der = Self::encode_sequence(encoded);
        let pem = base64::encode(der.clone());
        let formatted = format!(
            "-----BEGIN RSA PRIVATE KEY-----\n{}\n-----END RSA PRIVATE KEY-----",
            pem.chars().collect::<Vec<_>>().chunks(64).map(|c| c.iter().collect::<String>()).collect::<Vec<_>>().join("\n")
        );
        _ASN1Key { value: formatted }
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

///RSA Signing: S = M^d mod n
fn message_sign(message: &[u8], key_pair: &KeyPair) -> Result<BigUint, Box<dyn Error>> {
    if message.is_empty() {
        return Err("The message can't be empty".into());
    }

    let m = BigUint::from_bytes_be(message);

    if &m >= &key_pair.n {
        return Err("The message is too big".into());
    }

    Ok(m.modpow(&key_pair.d, &key_pair.n))
}

///RSA  Verification: M' = S^e mod n
fn signature_verification(message: &[u8], signature: &BigUint, key_pair: &KeyPair) -> Result<bool, Box<dyn Error>> {
    if message.is_empty() {
        return Err("The message can't be empty".into());
    }
    let m_verif = signature.modpow(&key_pair.e, &key_pair.n);
    let message_b = BigUint::from_bytes_be(message);

    Ok(m_verif == message_b)
}
