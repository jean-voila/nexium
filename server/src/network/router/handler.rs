use super::{
    http::{request::Request, response::Response, status::Status},
    routes::ping,
};
use std::{io::Write, net::TcpStream};

pub fn handler(stream: &mut TcpStream) {
    println!("New connection: {}", stream.peer_addr().unwrap());

    let req = match Request::from_stream(stream) {
        Ok(r) => r,
        Err(e) => {
            let res = Response::new(Status::BadRequest, e);
            stream.write_all(res.to_string().as_bytes());
            stream.flush();
            return;
        }
    };

    println!("method: {}", req.method);
    println!("path: {}", req.path);
    println!("header:");
    for (key, val) in req.headers.iter() {
        println!("'{key}': '{val}'");
    }

    println!("------------------");

    match req.path.as_str() {
        "/" => (),
        "/ping" => {
            ping::handler(stream, &req);
            return;
        }
        _ => {
            let res = Response::new(Status::NotFound, "");
            stream.write_all(res.to_string().as_bytes());
            stream.flush();
            return;
        }
    }

    let mut res = Response::new(Status::BadRequest, "");
    // let json = json::object! {
    //     "n": 1,
    //     "oidfh": "jbsdf"
    // };
    // let mut res = Response::new(Status::BadRequest, json.dump());
    // res.set_code(200);
    res.set_header("Content-Type", "text/plain");
    res.set_header("Content-Size", "1623");
    stream.write_all(res.to_string().as_bytes());
    stream.flush();
}
