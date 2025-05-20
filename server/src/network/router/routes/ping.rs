use super::super::http::{response::Response, status::Status};
use crate::network::router::http::request::Request;

pub fn handler(req: &mut Request) {
    let mut res = Response::new(Status::Ok, "pong");
    res.set_header("content-type", "text/plain");
    let _ = req.send(&res);
}
