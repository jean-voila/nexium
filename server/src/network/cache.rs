use rand::Rng;
use std::collections::HashMap;
use tokio::task::JoinHandle;

pub struct ServerCache {
    pub conn: HashMap<u16, JoinHandle<()>>,
}

impl ServerCache {
    pub fn new() -> Self {
        Self {
            conn: HashMap::new(),
        }
    }

    pub fn generate_id(&self) -> u16 {
        let mut r;
        loop {
            let mut rng = rand::rng();
            r = rng.random();

            if !self.conn.contains_key(&r) {
                break;
            }
        }
        return r;
    }

    pub fn add_conn(&mut self, id: u16, handle: JoinHandle<()>) -> u16 {
        self.conn.insert(id, handle);
        return id;
    }

    pub fn remove_conn(&mut self, id: u16) {
        self.conn.remove(&id);
    }

    pub async fn clear(&mut self) {
        println!("Clearing {} connections", self.conn.len());
        for (id, handle) in self.conn.drain() {
            // println!("Waiting for connection to finish...");
            println!("Removing connection ID: {}", id);
            let _ = handle.await;
            // println!("Connection finished");
        }
    }
}
