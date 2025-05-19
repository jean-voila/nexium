// use super::header::Header;
use super::status::Status;

// #[derive(Default)]
pub struct Response {
    code: Status,
    // headers: Vec<Header>,
    headers: Vec<String>,
}

impl Response {
    pub fn new(code: Status) -> Self {
        Self {
            code,
            headers: vec![],
            // ..Default::default()
        }
    }

    pub fn set_code(&mut self, code: Status) {
        self.code = code;
    }

    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.push(format!("{name}:{value}"));
    }

    pub fn to_buffer<'a>(&self) -> String {
        let headers = self.headers.join("\r\n");
        // dbg!(self.code);
        println!("code: {}", self.code);
        dbg!(&headers);
        format!("HTTP/1.1 {}\r\n{headers}\r\n\r\nTODO", self.code)
    }
}
