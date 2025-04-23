use std::net::{TcpListener, TcpStream};

pub struct _Server {
    port: u16,
}

fn _handle_client(_stream: TcpStream) {
    todo!();
}

impl _Server {
    fn _start(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))?;

        // accept connections and process them serially
        for stream in listener.incoming() {
            _handle_client(stream?);
        }
        Ok(())
    }
}
