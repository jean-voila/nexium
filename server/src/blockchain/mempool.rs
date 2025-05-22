use super::blockchain::Blockchain;
use nexium::blockchain::transaction::Transaction;

const TRANSACTION_COUNT: usize = 10;

struct Mempool<'a> {
    blockchain: &'a Blockchain,
    data: Vec<Transaction>,
}

impl<'a> Mempool<'a> {
    pub fn init(blockchain: &'a Blockchain) -> Self {
        Self {
            blockchain,
            data: vec![],
        }
    }

    pub fn add(&mut self, transaction: Transaction) {
        self.data.push(transaction);
        if self.data.len() > TRANSACTION_COUNT {
            println!("Mempool is full.");
        }
    }
}
