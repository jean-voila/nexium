#[cfg(test)]
use super::*;

#[test]
fn test_random() {
    let n1: u128 = 0;
    let n2: u128 = 100;

    for _ in 0..10 {
        let r: u128 = randint(n1, n2);
        assert!(r >= n1 && r <= n2);
    }
}
