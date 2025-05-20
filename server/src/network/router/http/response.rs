use super::status::Status;

pub struct Response {
    status: Status,
    headers: Vec<String>,
    body: String,
}

impl Response {
    pub fn new<T>(code: Status, body: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            status: code,
            headers: vec![],
            body: body.into(),
        }
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.push(format!("{name}:{value}"));
    }

    pub fn to_string(&self) -> String {
        if self.headers.len() == 0 {
            format!("HTTP/1.1 {}\r\n\r\n{}", self.status, self.body)
        } else {
            let headers = self.headers.join("\r\n");
            format!(
                "HTTP/1.1 {}\r\n{headers}\r\n\r\n{}",
                self.status, self.body
            )
        }

        //
        // same

        // let mut headers = String::new();
        // if self.headers.len() > 0 {
        //     headers.push_str("\r\n");
        //     headers.push_str(self.headers.join("\r\n").as_str());
        // }
        // format!("HTTP/1.1 {}{headers}\r\n\r\n{}", self.status, self.body)
    }
}
