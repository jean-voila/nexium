use super::super::http::{
    request::Request, response::Response, status::Status,
};
use std::{io::Write, net::TcpStream};

pub fn handler(stream: &mut TcpStream, req: &Request) {
    let sp: Vec<String> = req.path.split("/").map(|e| e.to_string()).collect();
    let user = &sp[2];

    if user.is_empty() {
        let mut res = Response::new(Status::BadRequest, "");
        // res.set_header("content-type", "text/json");
        stream.write_all(res.to_string().as_bytes());
        stream.flush();
        return;
    }

    let json = json::object! {
        "balance"=> 1000
    };

    let mut res = Response::new(Status::Ok, json.dump());
    res.set_header("content-type", "text/json");
    stream.write_all(res.to_string().as_bytes());
    stream.flush();
}
