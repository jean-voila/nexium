use crate::{
    blockchain::blockchain::Blockchain,
    network::http::{request::Request, response::Response, status::Status},
};
use std::io::Read;

pub async fn handler(
    req: Request,
    mut res: Response,
) -> Result<(), std::io::Error> {
    let mut start = match req.query.get("start") {
        Some(s) => {
            match s.parse::<u64>() {
                Ok(x) => x,
                Err(_) => {
                    res.status = Status::BadRequest;
                    return res
                    .send(b"Invalid start value, must be a non-negative integer")
                    .await;
                }
            }
        }
        None => 0,
    };

    let mut end = match req.query.get("end") {
        Some(e) => match e.parse::<u64>() {
            Ok(x) => x,
            Err(_) => {
                res.status = Status::BadRequest;
                return res
                    .send(b"Invalid end value, must be a non-negative integer")
                    .await;
            }
        },
        None => u64::MAX, // Default to the end of the file
    };

    if start > end {
        res.status = Status::BadRequest;
        return res
            .send(b"Invalid range: start cannot be greater than end")
            .await;
    }

    let mut file = match Blockchain::open_file(true, false, false) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open blockchain file: {}", e);
            res.status = Status::InternalServerError;
            return res.send(b"Failed to open blockchain file").await;
        }
    };

    match file.metadata() {
        Ok(metadata) => {
            let full_size = metadata.len();
            end = end.min(full_size);
            let size = full_size.min(end - start);
            res.set_header("Content-Length", &size.to_string());
            res.set_header("Content-Type", "application/octet-stream");
        }

        Err(e) => {
            eprintln!("Failed to get metadata of blockchain file: {}", e);
            res.status = Status::InternalServerError;
            return res
                .send(b"Failed to get metadata of blockchain file")
                .await;
        }
    };

    // println!("start: {start}");
    // println!("end: {end}");
    // println!("To read: {}", end - start);

    let mut buffer = Vec::with_capacity(1024 * 64); // 64 KB buffer
    let mut diff;
    loop {
        diff = (end - start).min(1024 * 64) as usize; // Ensure we read at most 64 KB

        // println!("Reading {} bytes from blockchain file", diff);
        if diff == 0 {
            res.send_empty().await?;
            break; // No more data to read
        }

        buffer.resize(diff, 0); // Resize to maintain capacity

        match file.read(&mut buffer) {
            Ok(bytes_read) => {
                // println!("Read {} bytes from blockchain file", bytes_read);
                start += bytes_read as u64;

                if bytes_read == 0 {
                    res.send_empty().await?;
                    break; // EOF
                }
            }

            Err(e) => {
                eprintln!("Failed to read from blockchain file: {}", e);
                // res.status = Status::InternalServerError;
                return res.send(b"Failed to read from blockchain file").await;
            }
        };

        res.send(&buffer).await?;
        buffer.clear(); // Clear the buffer for the next read
    }

    Ok(())
}
