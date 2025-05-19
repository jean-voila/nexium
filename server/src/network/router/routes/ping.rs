use super::super::http::{
    request::Request, response::Response, status::Status,
};
use std::{io::Write, net::TcpStream};

pub fn handler(stream: &mut TcpStream, req: &Request) {
    let res = Response::new(Status::Ok, "pong");
    stream.write_all(res.to_string().as_bytes());
    stream.flush();
}
