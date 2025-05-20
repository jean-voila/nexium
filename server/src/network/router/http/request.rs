use super::response::Response;
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
};

const READ_SIZE: usize = 2048;

pub struct Request<'a> {
    pub method: String,
    pub path_query: String,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
    stream: &'a mut TcpStream,
}

impl<'a> Request<'a> {
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
        let path_query = info[1].to_string();
        (method, path_query)
    }

    fn parse_header(line: &String) -> (String, String) {
        let v: Vec<_> = line.split(": ").collect();
        (v[0].to_string(), v[1].to_string())
    }

    fn read_req(mut stream: &mut TcpStream) -> Result<String, String> {
        let mut buff = [0; READ_SIZE];
        let mut res = String::new();

        // read until end of stream
        // loop {
        //     let r = stream.read(&mut buff).unwrap();
        //     let s = String::from_utf8(buff.to_vec()).expect("convertion failed");
        //     res.push_str(s.as_str());

        //     if r < READ_SIZE {
        //         break;
        //     }
        // }
        ///////////////////////////

        let r = stream.read(&mut buff).unwrap();
        if r == READ_SIZE {
            return Err(String::from("Request too long"));
        }

        let s = String::from_utf8(buff.to_vec()).expect("convertion failed");
        res.push_str(s.as_str());
        return Ok(res);
    }

    pub fn from_stream(stream: &'a mut TcpStream) -> Result<Self, String> {
        let raw = match Request::read_req(stream) {
            Ok(r) => r,
            Err(e) => return Err(e),
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
            req.headers.insert(key, value);
            lines.rotate_left(1);
        }
        lines.rotate_left(1);

        req.body = match lines[0].find("\0") {
            Some(i) => lines[0].drain(0..i).collect(),
            None => lines[0].to_string(),
        };

        return Ok(req);
    }

    pub fn _send(stream: &mut TcpStream, res: &Response) -> Result<(), String> {
        let buf = res.to_string();
        match stream.write_all(buf.as_bytes()) {
            Ok(()) => match stream.flush() {
                Ok(()) => Ok(()),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn send(&mut self, res: &Response) -> Result<(), String> {
        Request::_send(self.stream, res)
    }
}
