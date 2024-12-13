#[cfg(test)]
use super::*;

#[test]
fn test_gen_prime() {
    let p: u128 = _gen_prime();
    assert_eq!(_is_prime(p), true);
}

#[test]
fn test_gcd() {
    assert_eq!(_gcd(10, 5), 5);
    assert_eq!(_gcd(10, 3), 1);
    assert_eq!(_gcd(10, 0), 10);
}

#[test]
fn test_mod_inverse() {
    assert_eq!(_mod_inverse(3, 11), Some(4));
    assert_eq!(_mod_inverse(3, 10), Some(7));
    assert_eq!(_mod_inverse(3, 3), None);
    assert_eq!(_mod_inverse(3, 2), Some(1));
    assert_eq!(_mod_inverse(3, 1), None);
}

#[test]
fn test_is_prime() {
    assert_eq!(_is_prime(1), false);
    assert_eq!(_is_prime(2), true);
    assert_eq!(_is_prime(3), true);
    assert_eq!(_is_prime(4), false);
    assert_eq!(_is_prime(5), true);
    assert_eq!(_is_prime(6), false);
    assert_eq!(_is_prime(7), true);
    assert_eq!(_is_prime(8), false);
    assert_eq!(_is_prime(9), false);
    assert_eq!(_is_prime(10), false);
    assert_eq!(_is_prime(11), true);
    assert_eq!(_is_prime(12), false);
    assert_eq!(_is_prime(13), true);
    assert_eq!(_is_prime(14), false);
    assert_eq!(_is_prime(15), false);
    assert_eq!(_is_prime(16), false);
    assert_eq!(_is_prime(17), true);
    assert_eq!(_is_prime(18), false);
    assert_eq!(_is_prime(19), true);
    assert_eq!(_is_prime(20), false);
    assert_eq!(_is_prime(21), false);
    assert_eq!(_is_prime(22), false);
    assert_eq!(_is_prime(23), true);
    assert_eq!(_is_prime(24), false);
    assert_eq!(_is_prime(25), false);
    assert_eq!(_is_prime(26), false);
    assert_eq!(_is_prime(27), false);
    assert_eq!(_is_prime(28), false);
    assert_eq!(_is_prime(29), true);
    assert_eq!(_is_prime(30), false);
    assert_eq!(_is_prime(31), true);
    assert_eq!(_is_prime(32), false);
    assert_eq!(_is_prime(33), false);
    assert_eq!(_is_prime(34), false);
    assert_eq!(_is_prime(35), false);
    assert_eq!(_is_prime(36), false);
    assert_eq!(_is_prime(37), true);
    assert_eq!(_is_prime(38), false);
    assert_eq!(_is_prime(39), false);
    assert_eq!(_is_prime(40), false);
    assert_eq!(_is_prime(41), true);
    assert_eq!(_is_prime(563), true);
    assert_eq!(_is_prime(587), true);
    assert_eq!(_is_prime(599), true);
    assert_eq!(_is_prime(617), true);
    assert_eq!(_is_prime(709), true);
    assert_eq!(_is_prime(739), true);
    assert_eq!(_is_prime(773), true);
    assert_eq!(_is_prime(797), true);
    assert_eq!(_is_prime(859), true);
}