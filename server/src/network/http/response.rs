use super::status::Status;
use std::collections::HashMap;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub struct Response {
    pub status: Status,
    headers: HashMap<String, String>,
    headers_sent: bool,
    stream: TcpStream,
}

#[allow(dead_code)]
impl Response {
    pub fn new(stream: TcpStream) -> Response {
        Self {
            status: Status::Ok,
            headers: HashMap::new(),
            headers_sent: false,
            stream,
        }
    }

    pub fn set_header<T>(&mut self, key: T, value: T)
    where
        T: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
    }

    pub fn delete_header<T>(&mut self, key: T)
    where
        T: Into<String>,
    {
        self.headers.remove(&key.into());
    }

    fn generate(&self) -> String {
        let mut response = format!("HTTP/1.1 {}\r\n", self.status);
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        response.push_str("\r\n");
        response
    }

    async fn send_headers(&mut self) -> Result<(), std::io::Error> {
        let response = self.generate();
        self.stream.write_all(response.as_bytes()).await?;
        self.headers_sent = true;
        Ok(())
    }

    pub async fn send(&mut self, body: &[u8]) -> Result<(), std::io::Error> {
        if !self.headers_sent {
            self.send_headers().await?;
        }
        self.stream.write_all(body).await?;
        self.stream.flush().await
    }

    pub async fn send_empty(mut self) -> Result<(), std::io::Error> {
        if !self.headers_sent {
            self.send_headers().await?;
        }
        self.stream.flush().await
    }
}
