use crate::network::http::{
    request::Request, response::Response, status::Status,
};

pub async fn handler(
    req: Request,
    mut res: Response,
) -> Result<(), std::io::Error> {
    res.status = Status::Ok;
    res.send(b"Nodes list is not implemented yet").await
}
