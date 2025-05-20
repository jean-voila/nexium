use super::{
    super::server::Server,
    http::{request::Request, response::Response, status::Status},
    routes::{
        check_nexium, get_balance, get_transactions, new_transaction, ping,
    },
};
use std::{io::Write, net::TcpStream};

pub fn handler(server: &Server, stream: &mut TcpStream) {
    println!("New connection: {}", stream.peer_addr().unwrap());

    let mut req = match Request::from_stream(stream) {
        Ok(r) => r,
        Err(e) => {
            let res = Response::new(Status::BadRequest, e);
            let _ = Request::_send(stream, &res);
            return;
        }
    };

    println!("method: {}", req.method);
    println!("path: {}", req.path);
    println!("path_query: {}", req.path_query);
    println!("query:");
    for (key, val) in req.query.iter() {
        println!("'{key}': '{val}'");
    }
    println!("------------------");
    println!("header:");
    for (key, val) in req.headers.iter() {
        println!("'{key}': '{val}'");
    }
    println!("------------------");
    println!("body length: {}", req.body.len());
    println!("body: {}", req.body);
    println!("------------------");

    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/ping") => {
            ping::handler(&mut req);
        }
        ("GET", "/nexium") => {
            check_nexium::handler(&mut req);
        }
        (method, path) if method == "GET" && path.starts_with("/balance/") => {
            get_balance::handler(&mut req, &server);
        }
        ("GET", "/transactions") => {
            get_transactions::handler(&mut req, &server);
        }
        ("POST", "/transaction") => {
            new_transaction::handler(&mut req, &server);
        }
        _ => {
            let res = Response::new(Status::NotFound, "");
            let _ = stream.write_all(res.to_string().as_bytes());
            let _ = stream.flush();
        }
    };

    // let mut res = Response::new(Status::BadRequest, "");

    //

    // let json = json::object! {
    //     "n": 1,
    //     "oidfh": "jbsdf"
    // };
    // let mut res = Response::new(Status::BadRequest, json.dump());
    // res.set_code(200);

    //

    // res.set_header("Content-Type", "text/plain");
    // res.set_header("Content-Size", "1623");
    // stream.write_all(res.to_string().as_bytes());
    // stream.flush();
}
