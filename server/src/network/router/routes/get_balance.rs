use super::super::http::{
    request::Request, response::Response, status::Status,
};
use nexium::{
    gitlab::{GitlabClient, TokenType},
    rsa::KeyPair,
};
use std::{io::Write, net::TcpStream};

pub fn handler(stream: &mut TcpStream, req: &Request) {
    let sp: Vec<String> = req.path.split("/").map(|e| e.to_string()).collect();
    let user = &sp[2];

    if user.is_empty() {
        let res = Response::new(Status::BadRequest, "");
        let _ = stream.write_all(res.to_string().as_bytes());
        let _ = stream.flush();
        return;
    }
    // println!("user: {user}");

    let json = json::object! {
        "balance"=> 1000
    };

    let gitlab = GitlabClient::new(String::new(), TokenType::Classic);
    let gitlab_keys = match gitlab.get_gpg_keys(user) {
        Ok(keys) => keys,
        Err(e) => {
            let res = Response::new(Status::NotFound, "");
            let _ = stream.write_all(res.to_string().as_bytes());
            let _ = stream.flush();
            return;
        }
    };
    let pub_key = gitlab_keys[0].as_str();

    let key = match KeyPair::pub_from_pem(pub_key, &user) {
        Ok(key) => key,
        Err(e) => {
            let res = Response::new(Status::InternalError, "");
            let _ = stream.write_all(res.to_string().as_bytes());
            let _ = stream.flush();
            return;
        }
    };
    let data = json.dump().as_bytes().to_vec();
    let crypted = match key.crypt(data) {
        Ok(res) => res,
        Err(e) => {
            let res = Response::new(Status::InternalError, "");
            let _ = stream.write_all(res.to_string().as_bytes());
            let _ = stream.flush();
            return;
        }
    };
    dbg!(crypted);

    //

    let mut res = Response::new(Status::Ok, json.dump());
    // res.set_header("content-type", "text/plain");
    res.set_header("content-type", "text/json");
    let _ = stream.write_all(res.to_string().as_bytes());
    let _ = stream.flush();
}
