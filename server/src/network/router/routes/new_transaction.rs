use nexium::blockchain::transaction::Transaction;

use crate::network::{
    router::http::{request::Request, response::Response, status::Status},
    server::Server,
};

pub fn handler(req: &mut Request, server: &mut Server) {
    // let key = match req.check(&mut server.cache) {
    //     Ok(k) => k,
    //     Err(e) => {
    //         let res = Response::new(Status::BadRequest, e);
    //         let _ = req.send(&res);
    //         return;
    //     }
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

    // dbg!(&tr);
    let mut message = tr.header.to_buffer().to_vec();
    message.extend(&tr.data);

    let key = match server.cache.get_key(
        &tr.header.get_login(),
        &tr.signature.to_string(),
        Some(&message),
    ) {
        Some(k) => k,
        None => {
            let res = Response::new(Status::BadRequest, "Invalid key");
            let _ = req.send(&res);
            return;
        }
    };

    match key.check_signature(message, &tr.signature) {
        Ok(res) => {
            if !res {
                let res =
                    Response::new(Status::BadRequest, "Invalid signature");
                let _ = req.send(&res);
                return;
            }
        }
        Err(_) => {
            let res =
                Response::new(Status::BadRequest, "Failed to check signature");
            let _ = req.send(&res);
            return;
        }
    }

    server.cache.blockchain.add_transaction(tr);

    let res = Response::new(Status::Ok, "");
    let _ = req.send(&res);
}
