pub const TRANSACTION_HEADER_SIZE: usize = 73;
// pub const TRANSACTION_SIZE: usize = 2;
// pub const TRANSACTION_TIMESTAMP: usize = 4;
// pub const TRANSACTION_FEES: usize = 2;
pub const TRANSACTION_EMITTER: usize = 64;
// pub const TRANSACTION_DATA_TYPE: usize = 1;
pub const SIGNATURE_SIZE: usize = 256;

// ClassicTransaction
pub const TRANSACTION_RECEIVER: usize = 64;
pub const DESCRIPTION_SIZE: usize = 256;
pub const CLASSIC_TRANSACTION_MIN_SIZE: usize = TRANSACTION_EMITTER + 4 + 1;
pub const CLASSIC_TRANSACTION_MAX_SIZE: usize =
    CLASSIC_TRANSACTION_MIN_SIZE + DESCRIPTION_SIZE;
