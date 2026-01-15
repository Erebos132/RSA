// This document is a collection of useful functions which are needed throughout the entire
// codebase

use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt, ToBigUint};
use num_traits::cast::FromPrimitive;
use num_traits::{One, Zero};

pub fn big(num: u128) -> BigUint {
    return BigUint::from_u128(num).unwrap();
}

pub fn ibig(num: i128) -> BigInt {
    return BigInt::from_i128(num).unwrap();
}

// Function for quickly calculating base^power mod modulus
pub fn pmod(base: &BigUint, power: &BigUint, modulus: &BigUint) -> BigUint {
    let mut base_clone = base.clone();
    let mut power_clone = power.clone();

    let mut result = big(1);

    while &power_clone >= &big(1) {
        // Odd
        if &power_clone % big(2) == big(1) {
            result = (result * &base_clone) % modulus;
            power_clone -= big(1);
        }
        // Even
        else {
            base_clone = (&base_clone * &base_clone) % modulus;
            power_clone /= big(2);
        }
    }
    return result;
}

// Calculate a*? = 1 mod m
// Written by Chatgpt
pub fn mod_inv(a_u: &BigUint, m_u: &BigUint) -> Option<BigUint> {
    let mut a = a_u.to_bigint().unwrap();
    let mut m = m_u.to_bigint().unwrap();

    let zero = BigInt::zero();
    let one = BigInt::one();

    let mut t = BigInt::zero();
    let mut new_t = BigInt::one();

    let mut r = m.clone();
    let mut new_r = &a % &m;

    while new_r != zero {
        let quotient = &r / &new_r;

        // (t, new_t) = (new_t, t - quotient * new_t)
        let temp_t = new_t.clone();
        new_t = &t - &quotient * &new_t;
        t = temp_t;

        // (r, new_r) = (new_r, r - quotient * new_r)
        let temp_r = new_r.clone();
        new_r = &r - &quotient * &temp_r;
        r = temp_r;
    }

    // gcd(a, m) != 1 → inverse does not exist
    if r != one {
        return None;
    }

    if t < zero {
        t += m;
    }

    t.to_biguint()
}

pub fn hash<T>(data: T) -> BigUint {
    return big(50);
}
