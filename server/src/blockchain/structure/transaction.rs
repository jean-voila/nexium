use super::{
    consts::{SIGNATURE_SIZE, TRANSACTION_HEADER_SIZE},
    data_type::DataType,
    transaction_header::{TransactionHeader, EMITTER},
};

pub type SIGNATURE = [u8; SIGNATURE_SIZE];

#[derive(Clone, PartialEq)]
pub struct Transaction {
    pub transaction_header: TransactionHeader,
    pub data: Vec<u8>,
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
    pub fn new(
        data: Vec<u8>,
        fees: u16,
        emitter: EMITTER,
        data_type: DataType,
        signature: SIGNATURE,
    ) -> Self {
        Self {
            transaction_header: TransactionHeader::new(
                data.len() as u16,
                fees,
                emitter,
                data_type,
            ),
            data,
            signature,
        }
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
        write!(
            f,
            "signature: {:?},\n",
            String::from_utf8(self.signature.to_vec()).unwrap()
        )?;
        write!(f, "}}")?;
        Ok(())
    }
}

pub fn transaction_vec_size(transactions: &Vec<Transaction>) -> u32 {
    transactions.iter().fold(0, |acc, t| acc + t.size())
}
