use crate::random::randint;

// use num_bigint::{BigUint, RandBigInt};

mod tests;
// for debugging purposes, we can set this to true.
// it will make the prime number generation easier,
// beginning with 5 digits prime numbers at most.
const EASY_CALC: bool = true;

// Returns a random, large prime number.
pub fn _gen_prime() -> u128 {
    let max_prime = match EASY_CALC {
        true => 99999,
        false => u128::MAX,
    };
    let mut p: u128 = randint(0, max_prime);

    // Ensure p is odd
    p |= 1;

    loop {
        if _is_prime(p) {
            return p;
        }

        p += 2;
        if p > u128::MAX / 2 {
            p = randint(0, u128::MAX / 2);
            p |= 1;
        }
    }
}

// Again, arithmetics..
// Returns the greatest common divisor of a and b.
pub fn _gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }

    return _gcd(b, a % b);
}

// So, after some researches, I found out that there is a huuge difference
// between inverse and modulo..
// Here is the algorithm to find the modular inverse of a number:
// https://www.codabrainy.com/inverse-modulaire/
// This also was a primary concept of my S2 Maths courses,
// and I remember my grade was something like 6, so.. understandable
// Takes a number a and a modulo m and returns the inverse modulo of a.
pub fn _mod_inverse(a: u128, m: u128) -> Option<u128> {
    for n in 0..m {
        if (a * n) % m == 1 {
            return Some(n);
        }

        if n == m - 1 {
            return None;
        }
    }
    return None;
}

// Again, AFIT nostalgia..
// Takes a number n and returns true if it is prime, false otherwise.
fn _is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i: u128 = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    return true;
}

// According to the French Miller-Rabin algorithm's page,
// here is the algorithm to follow to determine the probability
// that a number is prime :
// https://fr.wikipedia.org/wiki/Test_de_primalit%C3%A9_de_Miller-Rabin

//Miller-Rabin(n,k):          entrées : n un entier impair ≥ 3, k un entier ≥ 1
//répéter k fois :
//choisir a aléatoirement dans l'intervalle [2, n – 1]
//si Témoin_de_Miller(n,a)
//  renvoyer Faux                   sortie de boucle, n est composé
//Fin de boucle répéter
//renvoyer Vrai                     n est probablement //premier (si k est suffisamment grand)
