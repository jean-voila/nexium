use crate::network::{
    router::http::{request::Request, response::Response, status::Status},
    server::Server,
};

pub fn handler(req: &mut Request, server: &Server) {
    let sp: Vec<String> = req.path.split("/").map(|e| e.to_string()).collect();
    let login = &sp[2];

    if login.is_empty() {
        let res = Response::new(Status::BadRequest, "");
        let _ = req.send(&res);
        return;
    }
    // println!("login: {login}");

    let n = match req.query.get("n") {
        // Some(n) => n.parse::<u32>().unwrap_or(3),
        Some(n) => match n.parse::<u32>() {
            Ok(0) | Err(_) => 3,
            Ok(100..) => 100,
            Ok(x) => x,
        },
        None => 3,
    };

    // println!("n: {n}");

    let json = json::array![
        //
    ];
    dbg!(json.dump());

    let mut res = Response::new(Status::Ok, json.dump());
    res.set_header("content-type", "text/json");
    let _ = req.send(&res);
}
