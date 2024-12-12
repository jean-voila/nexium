use std::time::{SystemTime, UNIX_EPOCH};

// In this module, we will implement a random number generator.
// We will first do a simple time-based random number generator,
// then we will implement the Mersenne Twister algorithm (later).

// Returns a random number between n1 and n2.
pub fn randint(n1: u128, n2: u128) -> u128 {
    let nanos = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .subsec_nanos() as u128;


    let delta = nanos % (n2 - n1);

    return n1 + delta;
}