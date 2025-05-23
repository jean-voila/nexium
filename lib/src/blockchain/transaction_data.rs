use super::{
    consts::{DESCRIPTION_SIZE, TRANSACTION_RECEIVER},
    data_type::DataType,
};
use crate::blockchain::consts::{
    CLASSIC_TRANSACTION_MAX_SIZE, CLASSIC_TRANSACTION_MIN_SIZE,
};
// use serde::{Deserialize, Serialize};

pub type DESCRIPTION = [u8; DESCRIPTION_SIZE];
pub type RECEIVER = [u8; TRANSACTION_RECEIVER];

// #[derive(Debug)]
pub enum TransactionData {
    ClassicTransaction {
        // #[serde(with = "serde_receiver")]
        receiver: RECEIVER,
        amount: u32,
        has_description: bool,
        // #[serde(with = "serde_description")]
        description: DESCRIPTION,
    },
    Unknown {
        data: Vec<u8>,
    },
}

#[derive(Debug)]
pub enum TransactionDataError {
    InvalidData,
}

impl TransactionData {
    pub fn from_buffer(
        data_type: &DataType,
        buffer: &Vec<u8>,
    ) -> Result<Self, TransactionDataError> {
        match data_type {
            DataType::ClassicTransaction => {
                let amount_start = TRANSACTION_RECEIVER;
                let has_description_start = amount_start + 4;
                let description_start = has_description_start + 1;

                if buffer.len() < CLASSIC_TRANSACTION_MIN_SIZE {
                    return Err(TransactionDataError::InvalidData);
                }

                let mut receiver = [0; TRANSACTION_RECEIVER];
                let mut description = [0; DESCRIPTION_SIZE];

                receiver.copy_from_slice(&buffer[0..amount_start]);

                let amount = u32::from_le_bytes(
                    buffer[amount_start..has_description_start]
                        .try_into()
                        .unwrap(),
                );

                let has_description = buffer[has_description_start] == 1;

                if has_description {
                    if buffer.len() == description_start
                        || buffer.len() > CLASSIC_TRANSACTION_MAX_SIZE
                    {
                        return Err(TransactionDataError::InvalidData);
                    }

                    description.copy_from_slice(&buffer[description_start..]);
                }

                Ok(TransactionData::ClassicTransaction {
                    receiver,
                    amount,
                    has_description,
                    description,
                })
            }
            DataType::Unknown => Ok(TransactionData::Unknown {
                data: buffer.clone(),
            }),
        }
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        match self {
            TransactionData::ClassicTransaction {
                receiver,
                amount,
                has_description,
                description,
            } => {
                let amount_start = TRANSACTION_RECEIVER;
                let has_description_start = amount_start + 4;
                let description_start = has_description_start + 1;

                let mut buffer = vec![0; self.size()];
                buffer[..amount_start].copy_from_slice(receiver);
                buffer[amount_start..has_description_start]
                    .copy_from_slice(&amount.to_le_bytes());
                buffer[has_description_start] = *has_description as u8;
                if *has_description {
                    buffer[description_start..].copy_from_slice(description);
                }
                buffer
            }
            TransactionData::Unknown { data } => data.clone(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            TransactionData::ClassicTransaction {
                has_description,
                description,
                ..
            } => {
                let mut size = CLASSIC_TRANSACTION_MIN_SIZE;
                if *has_description {
                    size += description.len();
                }
                size
            }
            TransactionData::Unknown { data } => data.len(),
        }
    }

    pub fn get_type(&self) -> u8 {
        match self {
            TransactionData::ClassicTransaction { .. } => 1,
            TransactionData::Unknown { .. } => 0,
        }
    }
}

impl core::fmt::Debug for TransactionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;
        write!(f, "type: {:?},\n", self.get_type())?;
        match self {
            TransactionData::ClassicTransaction {
                receiver,
                amount,
                has_description,
                description,
            } => {
                write!(
                    f,
                    "receiver: {},\n",
                    String::from_utf8_lossy(receiver)
                )?;
                write!(f, "amount: {},\n", amount)?;
                write!(f, "has_description: {},\n", has_description)?;
                if *has_description {
                    write!(
                        f,
                        "description: {},\n",
                        String::from_utf8_lossy(description)
                    )?;
                }
            }
            TransactionData::Unknown { data } => {
                write!(f, "data: {:?},\n", data)?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

// mod serde_description {

//     use serde::{Deserialize, Deserializer, Serializer};

//     use crate::blockchain::consts::DESCRIPTION_SIZE;

//     use super::DESCRIPTION;

//     pub fn serialize<S>(
//         sig: DESCRIPTION,
//         serializer: S,
//     ) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let s = sig.iter().map(|e| *e as char).collect::<String>();
//         serializer.serialize_str(s.as_str())
//     }

//     pub fn deserialize<'de, D>(deserializer: D) -> Result<DESCRIPTION, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s: &str = Deserialize::deserialize(deserializer)?;
//         let mut description = [0; DESCRIPTION_SIZE];
//         description.copy_from_slice(s.as_bytes());
//         Ok(description)
//     }
// }
