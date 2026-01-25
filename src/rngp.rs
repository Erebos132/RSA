// File for the random prime number generation

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_traits::{One, Zero};
use rand::{Rng, RngCore};

use crate::gf;
use crate::gf::big;

// function needed for determining if a number is prime, probably inefficient, adapted from python
fn miller_test(n: &BigUint, d: &BigUint, r: u32, a: &BigUint) -> bool {
    let mut x = a.modpow(d, n);

    if &x == &big(1) || x == n - &big(1) {
        return true;
    }

    for _ in 0..(r - 1) {
        x = (&x * &x) % n;

        if x == big(1) {
            return false;
        }
        if x == n - big(1) {
            return true;
        }
    }
    return false;
}

// Function to check if a number is prime by doing tests on the number. More rounds = higher
// accuracy
pub fn check_prime<R: RngCore>(rng: &mut R, n: &BigUint, k: usize) -> bool {
    // Corner Cases
    if n <= &big(1) || n == &big(4) {
        return false;
    }
    if n <= &big(3) {
        return true;
    }

    // Check small primes
    const SMALL_PRIMES: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 31];
    for &p in &SMALL_PRIMES {
        let p = BigUint::from(p);
        if n == &p {
            return true;
        }
        if n % &p == BigUint::ZERO {
            return false;
        }
    }

    let mut d = n - BigUint::one();
    let mut r = 0u32;
    while (&d & BigUint::one()) == BigUint::ZERO {
        d >>= 1;
        r += 1;
    }

    for _ in 0..k {
        let testing_number = &rng.gen_biguint_range(&big(2), &(n - big(2)));
        if !miller_test(n, &d, r, &testing_number) {
            return false;
        }
    }
    return true;
}

// Generates many random numbers with bitlength and then checks if they are prime or not. If they
// are, the number is returned
pub fn get_prime_in_bitrange<R: RngCore>(rng: &mut R, bit_size: u64, rounds: usize) -> BigUint {
    let mut second_rng = rand::thread_rng();
    let mut random_number = rng.gen_biguint(bit_size);
    while !check_prime(&mut second_rng, &random_number, rounds)
        || &random_number < &big(2).pow(bit_size as u32 - 2)
    {
        random_number = rng.gen_biguint(bit_size);
    }
    return random_number;
}
