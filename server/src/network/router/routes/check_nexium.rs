use crate::network::{
    router::http::{request::Request, response::Response, status::Status},
    server::Server,
};

pub fn handler(req: &mut Request, server: &mut Server) {
    let key = match req.check(&mut server.cache) {
        Ok(data) => data,
        Err(e) => {
            let res = Response::new(Status::BadRequest, e);
            let _ = req.send(&res);
            return;
        }
    };

    let json = json::object! {
        login: server.login.clone(),
        // version: 0,
    };

    let data = json.dump();
    let crypted = match key.crypt(&data) {
        Ok(res) => res,
        Err(_) => {
            let res = Response::new(Status::InternalError, "");
            let _ = req.send(&res);
            return;
        }
    };

    let mut res = Response::new(Status::Ok, crypted);
    res.set_header("content-type", "text/plain");
    let _ = req.send(&res);
}
