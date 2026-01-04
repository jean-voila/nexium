use nexium::{
    blockchain::transaction::Transaction, defaults::TRANSACTION_COUNT,
};

pub struct Mempool {
    data: Vec<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn add(&mut self, transaction: Transaction) {
        self.data.push(transaction);
    }

    pub fn is_full(&self) -> bool {
        self.data.len() >= TRANSACTION_COUNT
    }

    pub fn dump(&mut self) -> Vec<Transaction> {
        self.data.drain(0..TRANSACTION_COUNT).collect()
    }

    /// Remove transactions that are included in a synced block
    pub fn remove_transactions(&mut self, transactions: &[Transaction]) {
        // Remove transactions by matching their signature (unique identifier)
        self.data.retain(|t| {
            !transactions.iter().any(|bt| bt.signature == t.signature)
        });
    }

    /// Clear all transactions from mempool
    pub fn clear(&mut self) {
        self.data.clear();
    }
}
