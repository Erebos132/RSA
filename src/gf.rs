// This document is a collection of useful functions which are needed throughout the entire
// codebase

use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt, ToBigUint};
use num_traits::cast::FromPrimitive;
use num_traits::{One, Zero};
use sha2::{Digest, Sha256};

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

pub fn hash_bytes(bytes: &[u8]) -> BigUint {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    BigUint::from_bytes_be(&hasher.finalize())
}

pub fn str_to_int(msg: &str) -> BigUint {
    let bytes = msg.as_bytes();
    return BigUint::from_bytes_be(bytes);
}

pub fn int_to_str(msg_int: &BigUint) -> String {
    let bytes = msg_int.to_bytes_be();
    String::from_utf8(bytes).unwrap()
}
