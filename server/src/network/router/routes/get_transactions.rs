use crate::network::{
    router::http::{request::Request, response::Response, status::Status},
    server::Server,
};

pub fn handler(req: &mut Request, server: &Server) {
    let json = json::array![
        //
    ];
    dbg!(json.dump());
    let mut res = Response::new(Status::Ok, json.dump());
    res.set_header("content-type", "text/json");
    let _ = req.send(&res);
}
