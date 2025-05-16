pub const BLOCK_HEADER_SIZE: usize = 82;
pub const HEADER_VERSION: usize = 2;
pub const HEADER_PREVIOUS_BLOCK_HASH_SIZE: usize = 32;
pub const HEADER_MERKLE_ROOT_SIZE: usize = 32;
// pub const HEADER_TIMESTAMP: usize = 4;
// pub const HEADER_DIFFICULTY_TARGET: usize = 4;
// pub const HEADER_NONCE: usize = 4;
// pub const HEADER_TRANSACTION_SIZE: usize = 4;

pub const TRANSACTION_HEADER_SIZE: usize = 73;
// pub const TRANSACTION_SIZE: usize = 2;
// pub const TRANSACTION_TIMESTAMP: usize = 4;
// pub const TRANSACTION_FEES: usize = 2;
pub const TRANSACTION_EMITTER: usize = 64;
// pub const TRANSACTION_DATA_TYPE: usize = 1;
pub const SIGNATURE_SIZE: usize = 256;
