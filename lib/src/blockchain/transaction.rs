use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use crate::rsa::KeyPair;

use super::{
    consts::{SIGNATURE_SIZE, TRANSACTION_HEADER_SIZE},
    data_type::DataType,
    transaction_header::TransactionHeader,
};

pub type SIGNATURE = [u8; SIGNATURE_SIZE];

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub header: TransactionHeader,
    pub data: Vec<u8>,
    #[serde(with = "serde_signature")]
    pub signature: BigUint,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            header: Default::default(),
            data: vec![],
            signature: BigUint::from(0u8),
        }
    }
}

impl Transaction {
    pub fn new<T>(
        data: Vec<u8>,
        fees: u16,
        emitter: T,
        data_type: DataType,
        key: &KeyPair,
    ) -> Result<Self, String>
    where
        T: Into<String>,
    {
        let header =
            TransactionHeader::new(data.len() as u16, fees, emitter, data_type);

        let mut buff = vec![0; TRANSACTION_HEADER_SIZE + data.len()];
        buff[0..TRANSACTION_HEADER_SIZE].copy_from_slice(&header.to_buffer());
        buff[TRANSACTION_HEADER_SIZE..].copy_from_slice(&data);
        // dbg!(&buff.len());

        let signature = match key.sign(buff) {
            Ok(sig) => sig,
            Err(_) => return Err("Error signing transaction".to_string()),
        };

        Ok(Self {
            header,
            data,
            signature,
        })
    }

    pub fn size(&self) -> u32 {
        (TRANSACTION_HEADER_SIZE + self.data.len() + SIGNATURE_SIZE) as u32
    }

    pub fn from_buffer(buff: &[u8]) -> Result<Self, String> {
        let data_start = TRANSACTION_HEADER_SIZE;
        let header_buff = match buff[0..data_start].try_into() {
            Ok(h) => h,
            Err(_) => return Err("Buffer too small".to_string()),
        };

        let header = TransactionHeader::from_buffer(&header_buff);
        let signature_start = data_start + header.transaction_size as usize;
        let signature_end = signature_start + SIGNATURE_SIZE;
        // check signature size
        if buff.len() < signature_end {
            return Err("Buffer too small".to_string());
        }

        let sig = BigUint::from_bytes_le(&buff[signature_start..signature_end]);

        Ok(Self {
            header,
            data: buff[data_start..signature_start].to_vec(),
            signature: sig,
        })
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let data_start = TRANSACTION_HEADER_SIZE;
        let signature_start =
            data_start + self.header.transaction_size as usize;

        let mut res = vec![
            0;
            TRANSACTION_HEADER_SIZE
                + self.header.transaction_size
                    as usize
                + SIGNATURE_SIZE
        ];

        res[0..TRANSACTION_HEADER_SIZE]
            .copy_from_slice(&self.header.to_buffer());

        res[TRANSACTION_HEADER_SIZE..signature_start]
            .copy_from_slice(&self.data);

        let mut sig = self.signature.to_bytes_le();
        if sig.len() < SIGNATURE_SIZE {
            sig.resize(SIGNATURE_SIZE, 0);
        };
        res[signature_start..].copy_from_slice(&sig);
        return res;
    }
}

impl core::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;
        write!(f, "header: {:?},\n", self.header)?;
        // write!(f, "transactions: [{:?}],\n", self.data)?;
        write!(f, "signature: {:?},\n", self.signature)?;
        write!(f, "}}")?;
        Ok(())
    }
}

pub fn transaction_vec_size(transactions: &Vec<Transaction>) -> u32 {
    transactions.iter().fold(0, |acc, t| acc + t.size())
}

mod serde_signature {
    use std::str::FromStr;

    use num_bigint::BigUint;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(sig: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = sig.to_string();
        serializer.serialize_str(s.as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        match BigUint::from_str(s) {
            Ok(res) => Ok(res),
            Err(_) => {
                return Err(serde::de::Error::custom("Invalid signature"))
            }
        }
    }
}
