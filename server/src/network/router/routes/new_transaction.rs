use nexium::blockchain::transaction::Transaction;

use crate::network::{
    router::http::{request::Request, response::Response, status::Status},
    server::Server,
};

pub fn handler(req: &mut Request, server: &mut Server) {
    // let x = "text/plain".to_string();
    // match req.headers.get("Content-Type") {
    //     Some(x) => {}
    //     None => {}
    // };

    let data = match server.key.decrypt_split(&req.body) {
        Ok(res) => res,
        Err(_) => {
            let res = Response::new(Status::BadRequest, "Invalid data");
            let _ = req.send(&res);
            return;
        }
    };

    let tr: Transaction = match serde_json::from_str(&data) {
        Ok(obj) => obj,
        Err(e) => {
            let res = Response::new(Status::BadRequest, e.to_string());
            let _ = req.send(&res);
            return;
        }
    };
    dbg!(&tr);

    server.blockchain.add_transaction(tr);

    let res = Response::new(Status::Ok, "");
    let _ = req.send(&res);
}
