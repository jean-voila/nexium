use crate::network::{
    router::http::{request::Request, response::Response, status::Status},
    server::Server,
};

pub fn handler(req: &mut Request, server: &mut Server) {
    let sp: Vec<String> = req.path.split("/").map(|e| e.to_string()).collect();
    let login = &sp[2];

    if login.is_empty() {
        let res = Response::new(Status::BadRequest, "");
        let _ = req.send(&res);
        return;
    }
    println!("login: {login}");

    let n = match req.query.get("n") {
        Some(n) => match n.parse::<usize>() {
            Ok(0) | Err(_) => 3,
            Ok(100..) => 100,
            Ok(x) => x,
        },
        None => 3,
    };
    println!("n: {n}");

    let key = match req.check(&mut server.cache) {
        Ok(data) => data,
        Err(e) => {
            let res = Response::new(Status::BadRequest, e);
            let _ = req.send(&res);
            return;
        }
    };

    let mut arr = json::array![];
    let mut hash = server.blockchain.last_hash;

    loop {
        let b = match server.blockchain.get_block(&hash) {
            Ok(b) => b,
            Err(_) => {
                let res = Response::new(Status::BadRequest, "Invalid block");
                let _ = req.send(&res);
                return;
            }
        };

        for tr in b.transactions.iter() {
            if tr.header.get_login() == *login {
                let obj = match serde_json::to_string(&tr) {
                    Ok(obj) => obj,
                    Err(_) => {
                        let res = Response::new(
                            Status::BadRequest,
                            "Failed to parse transaction",
                        );
                        let _ = req.send(&res);
                        return;
                    }
                };

                match arr.push(obj) {
                    Ok(_) => {}
                    Err(_) => {
                        let res = Response::new(
                            Status::BadRequest,
                            "Failed to add transaction object",
                        );
                        let _ = req.send(&res);
                        return;
                    }
                }

                if arr.len() >= n {
                    break;
                }
            }
        }

        if arr.len() >= n {
            break;
        }

        hash = b.header.previous_block_hash;

        match server.blockchain.cache.get(&hash) {
            Some(0) => {
                // end of blockchain
                break;
            }
            Some(_) => {} // continue
            None => {
                // block not found in cache
                let res = Response::new(Status::BadRequest, "Invalid block");
                let _ = req.send(&res);
                return;
            }
        }
    }

    let data = arr.dump();
    let crypted = match key.crypt_split(&data) {
        Ok(res) => res,
        Err(_) => {
            let res = Response::new(Status::InternalError, "");
            let _ = req.send(&res);
            return;
        }
    };
    // dbg!(&crypted);

    let mut res = Response::new(Status::Ok, crypted);
    res.set_header("content-type", "text/plain");
    // let mut res = Response::new(Status::Ok, json.dump());
    // res.set_header("content-type", "text/json");
    let _ = req.send(&res);
}
