use super::method::Method;
use nexium::{defaults::SIG_SAMPLE, gitlab::GitlabClient, rsa::KeyPair};
use std::collections::HashMap;
use tokio::{io::AsyncReadExt, net::TcpStream};

const READ_SIZE: usize = 32768;

pub struct Request {
    pub method: Method,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    async fn read_raw(stream: &mut TcpStream) -> Result<Vec<u8>, String> {
        let mut buf = [0; READ_SIZE];

        let r = match stream.read(&mut buf).await {
            Ok(n) if n == 0 => return Err("Connection closed".to_string()),
            Ok(n) if n >= READ_SIZE => {
                return Err("Request too large".to_string())
            }
            Err(e) => return Err(format!("Failed to read from stream: {}", e)),
            Ok(n) => n,
        };

        let mut res = vec![0; r];
        res.copy_from_slice(&buf[..r]);
        Ok(res)
    }

    fn find_first_occ(buf: &[u8], needle: &[u8]) -> Result<usize, String> {
        let i = buf
            .windows(needle.len())
            .position(|win| win == needle)
            .ok_or("Needle not found in buffer")?;
        Ok(i)
    }

    fn parse_info(info_line: &[u8]) -> Result<(Method, &str), String> {
        let mut words = info_line.split(|&b| b == b' ');

        let method = match words.next() {
            Some(m) => Method::from_str(m).ok_or("Invalid HTTP method")?,
            None => Err("Missing HTTP method")?,
        };

        let full_path = match words.next() {
            Some(p) => {
                std::str::from_utf8(p).map_err(|_| "Invalid request path")?
            }
            None => Err("Missing request path")?,
        };

        Ok((method, full_path))
    }

    fn parse_path_query(full_path: &str) -> (String, HashMap<String, String>) {
        let mut query_params = HashMap::new();
        let (path, query_str) = match full_path.split_once('?') {
            Some((p, q)) => (p.to_string(), q),
            None => (full_path.to_string(), ""),
        };

        for param in query_str.split('&') {
            if let Some((key, value)) = param.split_once('=') {
                let key = key.trim().to_string();
                let value = value.trim().to_string();
                query_params.insert(key, value);
            }
        }

        (path, query_params)
    }

    fn parse_headers(
        headers: &[u8],
    ) -> Result<HashMap<String, String>, String> {
        let mut header_map = HashMap::new();
        for h in headers.split(|&b| b == b'\n') {
            if h.is_empty() {
                continue; // Skip empty lines
            }

            let i = match h.iter().position(|&b| b == b':') {
                Some(pos) => pos,
                None => continue, // Skip if no colon is found
            };

            let key = match String::from_utf8(h[..i].trim_ascii().to_vec()) {
                Ok(k) => k,
                Err(_) => continue, // Skip if key is not valid
            };

            let value =
                match String::from_utf8(h[i + 1..].trim_ascii().to_vec()) {
                    Ok(v) => v,
                    Err(_) => continue, // Skip if value is not valid
                };

            header_map.insert(key, value);
        }

        Ok(header_map)
    }

    pub async fn from_stream(stream: &mut TcpStream) -> Result<Self, String> {
        let buf = Self::read_raw(stream).await?;

        let header_end = Self::find_first_occ(&buf, b"\r\n\r\n")?;
        let req_head = &buf[..header_end + 4];

        println!("Request Head: {:?}", String::from_utf8_lossy(req_head));
        println!(
            "Request Body: {:?}",
            String::from_utf8_lossy(&buf[header_end + 4..])
        );

        let info_end = Self::find_first_occ(req_head, b"\r\n")?;
        let req_info_line = &req_head[..info_end];
        let req_headers = &req_head[info_end + 2..];

        let (method, full_path) = Self::parse_info(req_info_line)?;
        println!("Method: {}", method);
        println!("Full Path: {}", full_path);

        let (path, query_params) = Self::parse_path_query(full_path);
        println!("Path: {}", path);
        println!("Query Params: {:?}", query_params);

        let headers = Self::parse_headers(req_headers)?;
        println!("Headers: {:?}", headers);

        Ok(Self {
            method,
            path,
            query: query_params,
            headers,
            body: buf[header_end + 4..].to_vec(),
        })
    }

    pub async fn get_key(
        &self,
        gitlab: &mut GitlabClient,
    ) -> Result<KeyPair, String> {
        let login = match self.headers.get("login") {
            Some(l) => l,
            None => return Err(String::from("Missing Login header")),
        };

        let sig = match self.headers.get("sig-sample") {
            Some(s) => s,
            None => return Err(String::from("Missing Sig-Sample header")),
        };

        match gitlab.find_user_key(login, sig, SIG_SAMPLE).await {
            Some(key) => Ok(key),
            None => Err("Key not found".to_string()),
        }
    }
}
