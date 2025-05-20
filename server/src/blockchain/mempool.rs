use super::structure::block::Block;
use lazy_static::lazy_static;
use nexium::blockchain::transaction::Transaction;
use std::sync::Mutex;

lazy_static! {
    static ref MEMPOOL: Mutex<Vec<Transaction>> = Mutex::new(Vec::new());
}

pub fn add_mempool(tr: Transaction) -> Result<(), String> {
    let res = MEMPOOL.lock();
    match res {
        Ok(mut mempool) => {
            mempool.push(tr);
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn size() -> Result<usize, String> {
    let mutex = MEMPOOL.lock();
    match mutex {
        Ok(mempool) => Ok(mempool.len()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn create_block() -> Result<Block, String> {
    let mutex = MEMPOOL.lock();
    match mutex {
        Ok(mempool) => {
            let b =
                Block::new(1, [0_u8; 32], [0_u8; 32], 0, 0, mempool.to_vec());
            Ok(b)
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn show_mempool() {
    let mempool = MEMPOOL.lock().unwrap();
    print!("[");
    if mempool.len() > 0 {
        print!("{}", mempool[0].transaction_header.transaction_size);
        for tr in mempool.iter().skip(1) {
            print!(", {}", tr.transaction_header.transaction_size);
        }
    }
    println!("]",);
}
