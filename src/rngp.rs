use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::{Rng, RngCore};

use crate::gf;
use crate::gf::big;

fn miller_test(d: &BigUint, n: &BigUint, a: &BigUint) -> bool {
    let mut d_clone = d.clone();
    let mut x = gf::pmod(a, d, n);

    if &x == &big(1) || x == n - &big(1) {
        return true;
    }

    while (&d_clone != &(n - &big(1))) {
        x = x.pow(2) % n;
        d_clone *= &big(2);

        if x == big(1) {
            return false;
        }
        if x == n - big(1) {
            return true;
        }
    }
    return false;
}

pub fn check_prime<R: RngCore>(rng: &mut R, n: &BigUint, k: usize) -> bool {
    let mut d = n - &big(1);
    while &d % &big(2) == big(0) {
        d /= big(2);
    }

    for _ in 0..k {
        if !miller_test(
            &d,
            n,
            &(big(2) + &rng.gen_biguint_range(&big(2), &(n - big(2)))),
        ) {
            return false;
        }
    }
    return true;
}

pub fn get_prime_in_bitrange<R: RngCore>(rng: &mut R, bit_size: u64, rounds: usize) -> BigUint {
    let mut second_rng = rand::thread_rng();
    let mut random_number = rng.gen_biguint(bit_size);
    while !check_prime(&mut second_rng, &random_number, rounds)
        || &random_number < &big(2).pow(bit_size as u32 - 1)
    {
        random_number = rng.gen_biguint(bit_size);
    }
    return random_number;
}
