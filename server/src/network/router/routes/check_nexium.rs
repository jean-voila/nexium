use super::super::http::{response::Response, status::Status};
use std::{io::Write, net::TcpStream};

pub fn handler(stream: &mut TcpStream) {
    let data = json::object! {
        version: 0,
    };
    let mut res = Response::new(Status::Ok, data.dump());
    res.set_header("content-type", "text/json");
    let _ = stream.write_all(res.to_string().as_bytes());
    let _ = stream.flush();
}
