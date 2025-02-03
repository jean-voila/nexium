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
        let p: BigUint = BigUint::parse_bytes(p_prime.to_string().as_bytes(), 10).unwrap();

        let q_prime: num_primes::BigUint = Generator::new_prime(bit_length / 2);
        let q: BigUint = BigUint::parse_bytes(q_prime.to_string().as_bytes(), 10).unwrap();

        // N.B : We're not wondering about the perormance cost of using
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

    fn _to_asn1(&self) -> _ASN1Key {
        // ASN1 is a standard to encapsulate the informations about an RSA Key.
        // Please refer to the documentation (I've sent links on discord)
        todo!();
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
