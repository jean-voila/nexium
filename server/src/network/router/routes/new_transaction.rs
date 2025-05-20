use crate::network::{
    router::http::{request::Request, response::Response, status::Status},
    server::Server,
};

pub fn handler(req: &mut Request, server: &Server) {
    // req.body

    let res = Response::new(Status::Ok, "");
    let _ = req.send(&res);
}
