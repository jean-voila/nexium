use super::super::http::{
    request::Request, response::Response, status::Status,
};

pub fn handler(req: &mut Request) {
    let data = json::object! {
        version: 0,
    };
    let mut res = Response::new(Status::Ok, data.dump());
    res.set_header("content-type", "text/json");
    let _ = req.send(&res);
}
