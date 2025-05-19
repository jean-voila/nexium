use super::router::handler::handler;
use std::net::{TcpListener, ToSocketAddrs};

#[derive(Default)]
pub struct Server {
    // port: u16,
    // addr: ToSocketAddrs
    // listener: TcpListener,
}

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    // pub fn listen<A>(&mut self, addr: String, port: u16)
    pub fn listen<A>(&mut self, addr: A)
    where
        A: ToSocketAddrs,
    {
        // self.port = port;
        // let addr = format!("127.0.0.1:{}", port);
        // self.listener = TcpListener::bind(addr).expect("Failed to {addr}");
        let listener = TcpListener::bind(addr).expect("Failed to {addr}");
        // println!("Server listening on port {}", port);

        // for s in self.listener.incoming() {
        for s in listener.incoming() {
            match s {
                Ok(stream) => handler(&stream),
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}
