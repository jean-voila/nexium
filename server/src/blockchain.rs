use std::{fs::File, io::Read, vec};

const BLOCK_HEADER_SIZE: usize = 82;
const HEADER_VERSION: usize = 2;
const HEADER_PREVIOUS_BLOCK_HASH: usize = 32;
const HEADER_MERKLE_ROOT: usize = 32;
const HEADER_TIMESTAMP: usize = 4;
const HEADER_DIFFICULTY_TARGET: usize = 4;
const HEADER_NONCE: usize = 4;
const HEADER_TRANSACTION_SIZE: usize = 4;

const TRANSACTION_COUNT: usize = 10; //
const TRANSACTION_HEADER_SIZE: usize = 73;
const TRANSACTION_SIZE: usize = 2;
const TRANSACTION_TIMESTAMP: usize = 4;
const TRANSACTION_FEES: usize = 2;
const TRANSACTION_EMITTER: usize = 64;
const TRANSACTION_DATA_TYPE: usize = 1;
const SIGNATURE_SIZE: usize = 256;

// #[derive(Debug, Default, Clone, Copy)]
#[derive(Default, Clone, Copy)]
pub struct BlockHeader {
    version: u16,
    previous_block_hash: [u8; HEADER_PREVIOUS_BLOCK_HASH],
    merkle_root: [u8; HEADER_MERKLE_ROOT],
    timestamp: u32,
    difficulty_target: u32,
    nonce: u32,
    transactions_size: u32,
}

impl BlockHeader {
    fn from_buff(buff: [u8; BLOCK_HEADER_SIZE]) -> Self {
        BlockHeader {
            version: u16::from_be_bytes(buff[0..2].try_into().unwrap()),
            previous_block_hash: buff[2..34].try_into().unwrap(),
            merkle_root: buff[34..66].try_into().unwrap(),
            timestamp: u32::from_be_bytes(buff[66..70].try_into().unwrap()),
            difficulty_target: u32::from_be_bytes(
                buff[70..74].try_into().unwrap(),
            ),
            nonce: u32::from_be_bytes(buff[74..78].try_into().unwrap()),
            transactions_size: u32::from_be_bytes(
                buff[78..82].try_into().unwrap(),
            ),
        }
    }

    fn to_buffer(self) -> [u8; BLOCK_HEADER_SIZE] {
        let mut res = [0; BLOCK_HEADER_SIZE];
        res[0..2].copy_from_slice(&self.version.to_be_bytes());
        res[2..34].copy_from_slice(&self.previous_block_hash);
        res[34..66].copy_from_slice(&self.merkle_root);
        res[66..70].copy_from_slice(&self.timestamp.to_be_bytes());
        res[70..74].copy_from_slice(&self.difficulty_target.to_be_bytes());
        res[74..78].copy_from_slice(&self.nonce.to_be_bytes());
        res[78..82].copy_from_slice(&self.transactions_size.to_be_bytes());
        return res;
    }
}

#[derive(Debug, Default, Clone, Copy)]
enum DataType {
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
    transaction_size: u16,
    timestamp: u32,
    fees: u16,
    emitter: [u8; TRANSACTION_EMITTER],
    data_type: DataType,
}

#[derive(Clone)]
pub struct Transaction {
    transaction_header: TransactionHeader,
    // data: [u8],
    // data: &[u8],
    data: Vec<u8>,
    signature: [u8; SIGNATURE_SIZE],
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
    fn fill_from_buffer(&mut self, buff: &[u8]) -> usize {
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

    fn to_buffer(self) -> Vec<u8> {
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
    fn fill_from_buffer(&mut self, buff: &[u8; TRANSACTION_HEADER_SIZE]) {
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

#[derive(Default, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: [Transaction; TRANSACTION_COUNT],
}

impl Block {
    pub fn to_buffer(self) -> Vec<u8> {
        let mut res = vec![];
        res.extend_from_slice(&self.header.to_buffer());
        for t in self.transactions {
            res.extend_from_slice(&t.to_buffer());
        }
        return res;
    }

    fn extract_header(buff: &[u8]) {
        //
    }
}

impl core::fmt::Debug for BlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;
        write!(f, "  transactions_size: {},\n", self.transactions_size)?;
        write!(f, "  version: {},\n", self.version)?;
        write!(
            f,
            "  previous_block_hash: {:?},\n",
            String::from_utf8(self.previous_block_hash.to_vec()).unwrap()
        )?;
        write!(
            f,
            "  merkle_root: {:?},\n",
            String::from_utf8(self.merkle_root.to_vec()).unwrap()
        )?;
        write!(f, "  timestamp: {},\n", self.timestamp)?;
        write!(f, "  difficulty_target: {},\n", self.difficulty_target)?;
        write!(f, "  nonce: {},\n", self.nonce)?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl core::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;

        write!(f, "header: {:?},\n", self.header)?;
        write!(f, "transactions: [\n")?;
        for t in &self.transactions {
            write!(f, "{:?},\n", t)?;
        }
        write!(f, "],\n")?;
        write!(f, "}}")?;
        Ok(())
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

/// Read a block from the file
/// Returns the new position in the file
pub fn readBlock(file: &mut File, pos: usize) -> (Block, usize) {
    let head_buff = &mut [0_u8; BLOCK_HEADER_SIZE];
    let _ = file.read(head_buff);

    let mut block = Block {
        header: BlockHeader::from_buff(*head_buff),
        ..Default::default()
    };

    let tr_size = block.header.transactions_size;
    let mut tr_buff = &mut vec![0_u8; tr_size as usize];
    let _ = file.read(&mut tr_buff);

    let mut current_pos = 0;

    for i in 0..TRANSACTION_COUNT {
        current_pos +=
            block.transactions[i].fill_from_buffer(&tr_buff[current_pos..]);
    }

    // dbg!(tr_size, current_pos, tr_size == current_pos as u32);
    return (block, pos + BLOCK_HEADER_SIZE + tr_size as usize);
}

// pub fn writeBlock(file: &mut File, block: Block) {
//     let buff = block.to_buffer();
//     let _ = file.write(&buff).unwrap();
//     // let head_buff = block.header.to_buffer();
//     // let _ = file.write(&head_buff).unwrap();

//     // let mut tr_buff = vec![];
//     // for t in block.transactions {
//     //     tr_buff.extend(t.to_buffer());
//     // }
//     // let _ = file.write(&tr_buff).unwrap();
// }

pub fn _create_temp_block() -> Block {
    // let transactions:[Transaction;TRANSACTION_COUNT] = [Default::default();TRANSACTION_COUNT];
    let mut tr = vec![];
    for _ in 0..TRANSACTION_COUNT {
        let data = [0; 16253];
        tr.push(Transaction {
            transaction_header: TransactionHeader {
                transaction_size: 16253,
                timestamp: 1743171415,
                fees: 0,
                emitter: ['a' as u8; TRANSACTION_EMITTER],
                data_type: DataType::ClassicTransaction,
            },
            data: data.to_vec(),
            signature: ['a' as u8; SIGNATURE_SIZE],
        });
    }

    let tr_size = tr.iter().fold(0, |acc, t| {
        acc + size_of_val(&t.transaction_header)
            + t.data.len()
            + t.signature.len()
    });

    // dbg!(tr_size);

    Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: ['a' as u8; HEADER_PREVIOUS_BLOCK_HASH],
            merkle_root: ['a' as u8; HEADER_MERKLE_ROOT],
            timestamp: 1743171086,
            difficulty_target: 3,
            nonce: 5,
            transactions_size: tr_size as u32,
        },
        transactions: tr.try_into().unwrap(),
    }
}
