use super::super::http::{
    request::Request, response::Response, status::Status,
};
use std::{io::Write, net::TcpStream};

pub fn handler(stream: &mut TcpStream, req: &Request) {
    let json = json::array![
        //
    ];
    dbg!(json.dump());
    let mut res = Response::new(Status::Ok, json.dump());
    res.set_header("content-type", "text/json");
    stream.write_all(res.to_string().as_bytes());
    stream.flush();
}
