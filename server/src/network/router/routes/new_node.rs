use crate::network::http::{
    request::Request, response::Response, status::Status,
};

pub async fn handler(
    req: Request,
    mut res: Response,
) -> Result<(), std::io::Error> {
    res.status = Status::Ok;
    res.send(b"Post block is not implemented yet").await
}
