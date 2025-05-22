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
}
