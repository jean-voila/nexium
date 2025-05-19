use super::super::http::{
    request::Request, response::Response, status::Status,
};
use std::{io::Write, net::TcpStream};

pub fn handler(stream: &mut TcpStream, req: &Request) {
    // req.body

    let mut res = Response::new(Status::Ok, "");
    stream.write_all(res.to_string().as_bytes());
    stream.flush();
}
