use crate::network::{
    router::http::{request::Request, response::Response, status::Status},
    server::Server,
};
use nexium::rsa::KeyPair;

pub fn handler(req: &mut Request, server: &Server) {
    let sp: Vec<String> = req.path.split("/").map(|e| e.to_string()).collect();
    let login = &sp[2];

    if login.is_empty() {
        let res = Response::new(Status::BadRequest, "");
        let _ = req.send(&res);
        return;
    }
    // println!("user: {user}");

    // let balance = match server.cache.get(login) {
    //     Some(u) => u.balance,
    //     None => {
    //         let res = Response::new(Status::NotFound, "");
    //         let _ = req.send(&res);
    //         return;
    //     }
    // };

    let json = json::object! {
        "balance"=> 1000
    };

    let gitlab_keys = match server.gitlab.get_gpg_keys(login) {
        Ok(keys) => keys,
        Err(e) => {
            let res = Response::new(Status::NotFound, "");
            let _ = req.send(&res);
            return;
        }
    };

    let pub_key = gitlab_keys[0].as_str();

    let key = match KeyPair::pub_from_pem(pub_key, &login) {
        Ok(key) => key,
        Err(e) => {
            let res = Response::new(Status::InternalError, "");
            let _ = req.send(&res);
            return;
        }
    };
    let data = json.dump().as_bytes().to_vec();
    let crypted = match key.crypt(data) {
        Ok(res) => res,
        Err(e) => {
            let res = Response::new(Status::InternalError, "");
            let _ = req.send(&res);
            return;
        }
    };
    dbg!(crypted);

    //

    let mut res = Response::new(Status::Ok, json.dump());
    // res.set_header("content-type", "text/plain");
    res.set_header("content-type", "text/json");
    let _ = req.send(&res);
}
