pub const TRANSACTION_COUNT: usize = 10;
const TRANSACTION_HEADER_SIZE: usize = 73;
pub const TRANSACTION_EMITTER: usize = 64;
pub const SIGNATURE_SIZE: usize = 256;

#[derive(Debug, Default, Clone, Copy)]
pub enum DataType {
    #[default]
    Unknown = 0,
    ClassicTransaction = 1,
}

impl DataType {
    fn from_u8(t: u8) -> Self {
        match t {
            1 => DataType::ClassicTransaction,
            _ => DataType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TransactionHeader {
    pub transaction_size: u16,
    pub timestamp: u32,
    pub fees: u16,
    pub emitter: [u8; TRANSACTION_EMITTER],
    pub data_type: DataType,
}

#[derive(Clone)]
pub struct Transaction {
    pub transaction_header: TransactionHeader,
    // data: [u8],
    // data: &[u8],
    pub data: Vec<u8>,
    pub signature: [u8; SIGNATURE_SIZE],
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

impl Transaction {
    pub fn fill_from_buffer(&mut self, buff: &[u8]) -> usize {
        let data_start = TRANSACTION_HEADER_SIZE;
        self.transaction_header
            .fill_from_buffer(&buff[0..data_start].try_into().unwrap());

        let signature_start =
            data_start + self.transaction_header.transaction_size as usize;
        let signature_end = signature_start + SIGNATURE_SIZE;
        self.data = buff[data_start..signature_start]
            [0..self.transaction_header.transaction_size as usize]
            .to_vec();
        self.signature =
            buff[signature_start..signature_end].try_into().unwrap();
        return TRANSACTION_HEADER_SIZE
            + self.transaction_header.transaction_size as usize
            + SIGNATURE_SIZE;
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

impl TransactionHeader {
    pub fn fill_from_buffer(&mut self, buff: &[u8; TRANSACTION_HEADER_SIZE]) {
        self.transaction_size =
            u16::from_be_bytes(buff[0..2].try_into().unwrap());
        self.timestamp = u32::from_be_bytes(buff[2..6].try_into().unwrap());
        self.fees = u16::from_be_bytes(buff[6..8].try_into().unwrap());
        self.emitter = buff[8..72].try_into().unwrap();
        self.data_type = DataType::from_u8(buff[72]);
    }

    fn to_buffer(self) -> [u8; TRANSACTION_HEADER_SIZE] {
        let mut res = [0; TRANSACTION_HEADER_SIZE];
        res[0..2].copy_from_slice(&self.transaction_size.to_be_bytes());
        res[2..6].copy_from_slice(&self.timestamp.to_be_bytes());
        res[6..8].copy_from_slice(&self.fees.to_be_bytes());
        res[8..72].copy_from_slice(&self.emitter);
        res[72] = self.data_type as u8;
        return res;
    }
}

impl core::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;

        write!(f, "header: {:?},\n", self.transaction_header)?;
        write!(f, "transactions: [\n")?;
        // write!(f, "{:?},\n", self.data)?;
        write!(
            f,
            "signature: {:?},\n",
            String::from_utf8(self.signature.to_vec()).unwrap()
        )?;
        write!(f, "}}")?;
        Ok(())
    }
}
