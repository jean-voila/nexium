use super::super::http::{
    request::Request, response::Response, status::Status,
};
use std::{io::Write, net::TcpStream};

pub fn handler(stream: &mut TcpStream, req: &Request) {
    let mut res = Response::new(Status::Ok, "pong");
    res.set_header("content-type", "text/plain");
    stream.write_all(res.to_string().as_bytes());
    stream.flush();
}
