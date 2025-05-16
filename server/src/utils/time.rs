use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_time() -> u32 {
    let time = SystemTime::now().duration_since(UNIX_EPOCH);
    // let unix_time = time.unwrap().as_secs();
    match time {
        Ok(now) => now.as_secs() as u32,
        Err(e) => {
            eprintln!("Error getting current time: {}", e);
            std::process::exit(1);
        }
    }
}
