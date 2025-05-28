use nexium::rsa::KeyPair;

use crate::blockchain::cache::cache::Cache;

use super::response::Response;
use std::collections::HashMap;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const READ_SIZE: usize = 32768;

pub struct Request {
    pub method: String,
    pub path_query: String,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
    stream: TcpStream,
}

impl Request {
    fn parse_path_query(map: &mut HashMap<String, String>, query: &String) {
        for param in query.split("&") {
            let p: Vec<_> = param.split("=").collect();
            if p.len() == 2 {
                map.insert(p[0].to_string(), p[1].to_string());
            }
        }
    }

    fn parse_info(line: &String) -> (String, String) {
        let info: Vec<&str> = line.split_ascii_whitespace().collect();
        let method = info[0].to_string();
        let path_query = if info.len() > 1 {
            info[1].to_string()
        } else {
            String::new()
        };
        (method, path_query)
    }

    fn parse_header(line: &String) -> (String, String) {
        let v: Vec<_> = line.split(": ").collect();
        (v[0].to_string(), v[1].to_string())
    }

    async fn read_req(stream: &mut TcpStream) -> Result<String, String> {
        let mut buff = [0; READ_SIZE];

        let r = match stream.read(&mut buff).await {
            Ok(r) => r,
            Err(e) => {
                return Err(format!("Failed to read request: {}", e));
            }
        };

        if r == READ_SIZE {
            return Err(String::from("Request too long"));
        }

        let s = match String::from_utf8(buff.to_vec()) {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("Failed to convert request: {}", e));
            }
        };

        return Ok(s);
    }

    pub async fn from_stream(
        mut stream: TcpStream,
    ) -> Result<Self, (String, TcpStream)> {
        let raw = match Request::read_req(&mut stream).await {
            Ok(r) => r,
            Err(e) => return Err((e, stream)),
        };

        let mut lines: Vec<_> = raw.lines().map(|l| l.to_string()).collect();
        let (method, path_query) = Request::parse_info(&lines[0]);
        lines.rotate_left(1);

        let pq: Vec<&str> = path_query.split("?").collect();
        let path = pq[0].to_string();
        let mut query_map: HashMap<String, String> = HashMap::new();

        if pq.len() > 1 {
            let query = pq[1].to_string();
            Request::parse_path_query(&mut query_map, &query);
        }

        let mut req = Self {
            method,
            path_query,
            path,
            query: query_map,
            headers: HashMap::new(),
            body: String::new(),
            stream,
        };

        while lines[0] != "" {
            let line = &lines[0];
            let (key, value) = Request::parse_header(line);
            req.headers.insert(key.to_lowercase(), value);
            lines.rotate_left(1);
        }
        lines.rotate_left(1);

        req.body = match lines[0].find("\0") {
            Some(i) => lines[0].drain(0..i).collect(),
            None => lines[0].to_string(),
        };

        return Ok(req);
    }

    pub async fn check(&self, cache: &mut Cache) -> Result<KeyPair, String> {
        let login = match self.headers.get("login") {
            Some(l) => l,
            None => return Err(String::from("Missing Login header")),
        };

        let sig = match self.headers.get("sig-sample") {
            Some(s) => s,
            None => return Err(String::from("Missing Sig-Sample header")),
        };

        match cache.get_key(login, sig, None).await {
            Some(k) => Ok(k),
            None => {
                return Err(String::from("Invalid signature"));
            }
        }
    }

    pub async fn _send(
        mut stream: TcpStream,
        res: &Response,
    ) -> Result<(), String> {
        let buf = res.to_string();
        match stream.write_all(buf.as_bytes()).await {
            Ok(()) => match stream.flush().await {
                Ok(()) => Ok(()),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn send(self, res: &Response) -> Result<(), String> {
        Request::_send(self.stream, res).await
    }
}
