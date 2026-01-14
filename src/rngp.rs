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

pub fn check_prime(n: &BigUint, k: usize) -> bool {
    let mut rng = rand::rng();
    let mut d = n - &big(1);
    while &d % &big(2) == big(0) {
        d /= big(2);
    }

    for _ in 0..k {
        if !miller_test(
            &d,
            n,
            &(big(2) + &rng.gen_bigint_range(&big(2), &(n - big(2)))),
        ) {
            return false;
        }
    }
    return true;
}
