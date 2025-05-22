use nexium::defaults::SIG_SAMPLE;

use crate::network::{
    router::http::{request::Request, response::Response, status::Status},
    server::Server,
};

pub fn handler(req: &mut Request, server: &mut Server) {
    let sig = match server.key.sign(SIG_SAMPLE) {
        Ok(s) => s,
        Err(e) => {
            dbg!(e);
            let res = Response::new(Status::InternalError, "");
            let _ = req.send(&res);
            return;
        }
    };

    let json = json::object! {
        login: server.login.clone(),
        sigSample: sig.to_string(),
        // version: 0,
    };

    let key = match req.check(&mut server.cache) {
        Ok(data) => data,
        Err(e) => {
            let res = Response::new(Status::BadRequest, e);
            let _ = req.send(&res);
            return;
        }
    };

    let data = json.dump();
    let crypted = match key.crypt_split(&data) {
        Ok(res) => res,
        Err(e) => {
            dbg!(e);
            let res = Response::new(Status::InternalError, "");
            let _ = req.send(&res);
            return;
        }
    };

    let mut res = Response::new(Status::Ok, crypted);
    res.set_header("content-type", "text/plain");
    let _ = req.send(&res);
}
