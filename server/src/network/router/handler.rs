use super::http::{response::Response, status::Status};
use std::{io::Write, net::TcpStream};

pub fn handler(mut stream: &TcpStream) {
    println!("New connection: {}", stream.peer_addr().unwrap());

    let mut res = Response::new(Status::BadRequest);
    // res.set_code(200);
    res.set_header("Content-Type", "text/plain");
    res.set_header("Content-Size", "1623");
    stream.write_all(res.to_buffer().as_bytes());
    stream.flush();
}
