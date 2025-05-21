use serde::{Deserialize, Serialize};

use crate::{blockchain::consts::TRANSACTION_EMITTER, rsa::KeyPair};

use super::{
    consts::{SIGNATURE_SIZE, TRANSACTION_HEADER_SIZE},
    data_type::DataType,
    transaction_header::{TransactionHeader, EMITTER},
};

pub type SIGNATURE = [u8; SIGNATURE_SIZE];

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_header: TransactionHeader,
    pub data: Vec<u8>,
    #[serde(with = "serde_signature")]
    pub signature: SIGNATURE,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            signature: [0; SIGNATURE_SIZE],
            transaction_header: Default::default(),
            data: vec![],
        }
    }
}

impl Transaction {
    pub fn new<T>(
        data: Vec<u8>,
        fees: u16,
        emitter: T,
        data_type: DataType,
        key: KeyPair,
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

        let sig: SIGNATURE = match key.sign(buff) {
            Ok(sig_vec) => match sig_vec.to_bytes_be().try_into() {
                Ok(sig_u8) => sig_u8,
                Err(_) => {
                    return Err(
                        "Error converting signature to array".to_string()
                    )
                }
            },
            Err(_) => return Err("Error signing transaction".to_string()),
        };
        // let sig = [0; SIGNATURE_SIZE];
        // println!("signature: {:?}", sig);
        // dbg!(&sig.len());

        Ok(Self {
            transaction_header: header,
            data,
            signature: sig,
        })
    }

    pub fn size(&self) -> u32 {
        (TRANSACTION_HEADER_SIZE + self.data.len() + SIGNATURE_SIZE) as u32
    }

    pub fn from_buffer(buff: &[u8]) -> Self {
        let data_start = TRANSACTION_HEADER_SIZE;
        let header = TransactionHeader::from_buffer(
            &buff[0..data_start].try_into().unwrap(),
        );
        let signature_start = data_start + header.transaction_size as usize;
        let signature_end = signature_start + SIGNATURE_SIZE;
        // check signature size
        if buff.len() < signature_end {
            panic!("Buffer is too small for transaction");
        }
        Self {
            transaction_header: header,
            data: buff[data_start..signature_start].to_vec(),
            signature: buff[signature_start..signature_end].try_into().unwrap(),
        }
    }

    pub fn to_buffer(self) -> Vec<u8> {
        let data_start = TRANSACTION_HEADER_SIZE;
        let signature_start =
            data_start + self.transaction_header.transaction_size as usize;
        let mut res = vec![
            0;
            TRANSACTION_HEADER_SIZE
                + self.transaction_header.transaction_size
                    as usize
                + SIGNATURE_SIZE
        ];
        res[0..TRANSACTION_HEADER_SIZE]
            .copy_from_slice(&self.transaction_header.to_buffer());
        res[TRANSACTION_HEADER_SIZE..signature_start]
            .copy_from_slice(&self.data);
        res[signature_start..].copy_from_slice(&self.signature);
        return res;
    }
}

impl core::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;
        write!(f, "header: {:?},\n", self.transaction_header)?;
        // write!(f, "transactions: [{:?}],\n", self.data)?;
        write!(f, "signature: {:?},\n", self.signature.to_vec())?;
        write!(f, "}}")?;
        Ok(())
    }
}

pub fn transaction_vec_size(transactions: &Vec<Transaction>) -> u32 {
    transactions.iter().fold(0, |acc, t| acc + t.size())
}

mod serde_signature {
    use super::SIGNATURE;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(
        sig: &SIGNATURE,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match std::str::from_utf8(sig) {
            Ok(s) => s.trim_end_matches('\0'),
            Err(_) => "",
        };
        serializer.serialize_str(s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SIGNATURE, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let mut buf = [0u8; super::SIGNATURE_SIZE];
        let bytes = s.as_bytes();
        if bytes.len() > super::SIGNATURE_SIZE {
            return Err(serde::de::Error::custom("Signature string too long"));
        }
        buf[..bytes.len()].copy_from_slice(bytes);
        Ok(buf)
    }
}
