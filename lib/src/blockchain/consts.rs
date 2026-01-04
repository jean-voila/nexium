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

/// Estimate the total transaction size for a classic transaction
/// Used to calculate fees before creating the transaction
pub fn estimate_classic_transaction_size(has_description: bool) -> usize {
    let data_size = if has_description {
        CLASSIC_TRANSACTION_MAX_SIZE
    } else {
        CLASSIC_TRANSACTION_MIN_SIZE
    };
    TRANSACTION_HEADER_SIZE + data_size + SIGNATURE_SIZE
}

/// Calculate the fee cost in NEX for a given fees rate (µNEX/byte) and transaction size
/// Fees are defined as µNEX per byte (micro-NEX = 0.000001 NEX)
pub fn calculate_fee_cost(fees_per_byte: u16, transaction_size: usize) -> f64 {
    let size = transaction_size as f64;
    let fees = fees_per_byte as f64;
    (size * fees) / 1_000_000.0
}

/// Estimate the fee cost for a classic transaction
pub fn estimate_classic_transaction_fee(
    fees_per_byte: u16,
    has_description: bool,
) -> f64 {
    let size = estimate_classic_transaction_size(has_description);
    calculate_fee_cost(fees_per_byte, size)
}
