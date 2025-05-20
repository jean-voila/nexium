use super::{
    consts::{TRANSACTION_EMITTER, TRANSACTION_HEADER_SIZE},
    data_type::DataType,
};
use crate::utils::time::current_time;
use serde::{Deserialize, Serialize};

pub type EMITTER = [u8; TRANSACTION_EMITTER];

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TransactionHeader {
    pub transaction_size: u16,
    pub timestamp: u32,
    pub fees: u16,
    #[serde(with = "serde_emitter")]
    pub emitter: EMITTER,
    pub data_type: DataType,
}

impl Default for TransactionHeader {
    fn default() -> Self {
        Self {
            emitter: [0; TRANSACTION_EMITTER],
            data_type: DataType::Unknown,
            transaction_size: 0,
            timestamp: 0,
            fees: 0,
        }
    }
}

impl TransactionHeader {
    pub fn new(
        transaction_size: u16,
        fees: u16,
        emitter: EMITTER,
        data_type: DataType,
    ) -> Self {
        Self {
            transaction_size,
            timestamp: current_time(),
            fees,
            emitter,
            data_type,
        }
    }

    // pub fn fill_from_buffer(&mut self, buff: &[u8; TRANSACTION_HEADER_SIZE]) {
    //     self.transaction_size =
    //         u16::from_be_bytes(buff[0..2].try_into().unwrap());
    //     self.timestamp = u32::from_be_bytes(buff[2..6].try_into().unwrap());
    //     self.fees = u16::from_be_bytes(buff[6..8].try_into().unwrap());
    //     self.emitter = buff[8..72].try_into().unwrap();
    //     self.data_type = DataType::from_u8(buff[72]);
    // }

    pub fn from_buffer(buff: &[u8; TRANSACTION_HEADER_SIZE]) -> Self {
        TransactionHeader {
            transaction_size: u16::from_be_bytes(
                buff[0..2].try_into().unwrap(),
            ),
            timestamp: u32::from_be_bytes(buff[2..6].try_into().unwrap()),
            fees: u16::from_be_bytes(buff[6..8].try_into().unwrap()),
            emitter: buff[8..72].try_into().unwrap(),
            data_type: DataType::from_u8(buff[72]),
        }
    }

    pub fn to_buffer(self) -> [u8; TRANSACTION_HEADER_SIZE] {
        let mut res = [0; TRANSACTION_HEADER_SIZE];
        res[0..2].copy_from_slice(&self.transaction_size.to_be_bytes());
        res[2..6].copy_from_slice(&self.timestamp.to_be_bytes());
        res[6..8].copy_from_slice(&self.fees.to_be_bytes());
        res[8..72].copy_from_slice(&self.emitter);
        res[72] = self.data_type as u8;
        return res;
    }
}

impl core::fmt::Debug for TransactionHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;
        write!(f, "transaction_size: {},\n", self.transaction_size)?;
        write!(f, "timestamp: {},\n", self.timestamp)?;
        write!(f, "fees: {},\n", self.fees)?;
        // write!(f, "emitter: [")?;
        // for i in 0..TRANSACTION_EMITTER {
        //     write!(f, "{}, ", self.emitter[i])?;
        // }
        // write!(f, "],\n")?;
        write!(
            f,
            "emitter: {:?},\n",
            String::from_utf8(self.emitter.to_vec()).unwrap()
        )?;
        // write!(f, "data_type: {},\n", self.data_type)?;
        write!(f, "}}")?;
        Ok(())
    }
}

mod serde_emitter {
    use serde::{Deserialize, Deserializer, Serializer};

    use crate::blockchain::consts::TRANSACTION_EMITTER;

    pub fn serialize<S>(
        emitter: &[u8; TRANSACTION_EMITTER],
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert the emitter into a UTF-8 string, trimming null bytes
        let s = match std::str::from_utf8(emitter) {
            Ok(s) => s.trim_end_matches('\0'),
            Err(_) => "",
        };
        serializer.serialize_str(s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<[u8; TRANSACTION_EMITTER], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let mut buf = [0u8; TRANSACTION_EMITTER];
        let bytes = s.as_bytes();
        if bytes.len() > TRANSACTION_EMITTER {
            return Err(serde::de::Error::custom("Emitter string too long"));
        }
        buf[..bytes.len()].copy_from_slice(bytes);
        Ok(buf)
    }
}
