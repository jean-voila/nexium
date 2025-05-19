use std::{collections::HashMap, io::Read, net::TcpStream};

const READ_SIZE: usize = 2048;

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    fn parse_info(line: &String) -> (String, String) {
        let sp: Vec<&str> = line.split_ascii_whitespace().collect();
        (sp[0].to_string(), sp[1].to_string())
    }

    fn parse_header(line: &String) -> (String, String) {
        let v: Vec<_> = line.split(": ").collect();
        (v[0].to_string(), v[1].to_string())
    }

    fn read_req(mut stream: &TcpStream) -> Result<String, String> {
        let mut buff = [0; READ_SIZE];
        let mut res = String::new();

        // loop {
        //     let r = stream.read(&mut buff).unwrap();
        //     let s = String::from_utf8(buff.to_vec()).expect("convertion failed");
        //     res.push_str(s.as_str());

        //     if r < READ_SIZE {
        //         break;
        //     }
        // }

        let r = stream.read(&mut buff).unwrap();
        if r == READ_SIZE {
            return Err(String::from("Request too long"));
        }

        let s = String::from_utf8(buff.to_vec()).expect("convertion failed");
        res.push_str(s.as_str());
        return Ok(res);
    }

    pub fn from_stream(stream: &TcpStream) -> Result<Self, String> {
        let raw = match Request::read_req(&stream) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        let mut lines: Vec<_> = raw.lines().map(|l| l.to_string()).collect();
        let (method, path) = Request::parse_info(&lines[0]);
        lines.rotate_left(1);

        let mut req = Self {
            method,
            path,
            headers: HashMap::new(),
        };

        while lines[0] != "" {
            let line = &lines[0];
            let (key, value) = Request::parse_header(line);
            req.headers.insert(key, value);
            lines.rotate_left(1);
        }

        return Ok(req);
    }
}
