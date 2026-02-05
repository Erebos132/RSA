// This document is a collection of useful functions which are needed throughout the entire
// codebase

use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt, ToBigUint};
use num_traits::cast::FromPrimitive;
use num_traits::{One, Zero};
use sha2::{Digest, Sha256};

// Convert number (Rust) to a BigUInt
pub fn big(num: u128) -> BigUint {
    return BigUint::from_u128(num).unwrap();
}

pub fn unbig(num: &BigUint) -> u64 {
    return num.to_u64_digits()[0];
}

// Convert number (Rust) to a BigInt (can be negative)
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
            // Normal Expoentiation -> Just Multiply with base and reduce Power by 1
            result = (result * &base_clone) % modulus;
            power_clone -= big(1);
        }
        // Even
        else {
            // Square Base, Halve Exponent
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

// Use of Crate functionality: Return a hash (BigUint) of a bytes-array -> Basically everything if
// converted to bytes
pub fn hash_bytes(bytes: &[u8]) -> BigUint {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    BigUint::from_bytes_be(&hasher.finalize()[..])
}

// Custom Output-bitlength
pub fn hash_bytes_col(bytes: &[u8], output_bitlength: usize) -> Vec<u8> {
    let mut output_bytes = vec![];
    while output_bytes.len() * 8 < output_bitlength {
        let mut hasher = Sha256::new();
        hasher.update(&output_bytes);
        hasher.update(bytes);
        output_bytes.append(&mut hasher.finalize().to_vec());
    }
    return output_bytes[0..output_bitlength / 8].to_vec();
}

// Parser to parse a string of any length into a BigUint --> Works by taking byte-list of string
// and converting it to an integer -> UTF-8
pub fn str_to_int(msg: &str) -> BigUint {
    let bytes = msg.as_bytes();
    return BigUint::from_bytes_be(bytes);
}

// Reverse Parser to create a string from a BigUint (probably parsed before) --> Takes bytes and
// reinterprets them as String -> UTF-8
pub fn int_to_str(msg_int: &BigUint) -> String {
    let bytes = msg_int.to_bytes_be();
    String::from_utf8(bytes).unwrap()
}

// Written by Chatgpt, taking the nth root from a bigint
pub fn nth_root(n: &BigUint, k: u32) -> BigUint {
    use num_traits::One;

    let mut low = BigUint::zero();
    let mut high = BigUint::one() << ((n.bits() + k as u64 - 1) / k as u64);

    while low < high {
        let mid: BigUint = (&low + &high + 1u32) >> 1;
        if mid.pow(k) <= *n {
            low = mid;
        } else {
            high = mid - 1u32;
        }
    }
    low
}

pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(&a, &b)| a ^ b).collect()
}

// To Visualize big numbers: converting BigUint to Base64 (written by Chatgpt)
const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
pub fn base64_encode(num: &BigUint) -> String {
    let bytes = num.to_bytes_be();
    if bytes.is_empty() {
        return String::new();
    }

    let mut out = String::with_capacity((bytes.len() + 2) / 3 * 4);

    let mut i = 0;
    while i < bytes.len() {
        let b0 = bytes[i];
        let b1 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
        let b2 = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };

        let triple = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);

        out.push(BASE64_TABLE[((triple >> 18) & 0x3F) as usize] as char);
        out.push(BASE64_TABLE[((triple >> 12) & 0x3F) as usize] as char);

        if i + 1 < bytes.len() {
            out.push(BASE64_TABLE[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }

        if i + 2 < bytes.len() {
            out.push(BASE64_TABLE[(triple & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }

        i += 3;
    }

    out
}

fn b64_value(c: u8) -> Option<u8> {
    match c {
        b'A'..=b'Z' => Some(c - b'A'),
        b'a'..=b'z' => Some(c - b'a' + 26),
        b'0'..=b'9' => Some(c - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

pub fn base64_decode(s: String) -> Result<BigUint, &'static str> {
    let bytes = s.as_bytes();

    if bytes.len() % 4 != 0 {
        return Err("Invalid base64 length");
    }

    let mut out = Vec::with_capacity(bytes.len() / 4 * 3);

    let mut i = 0;
    while i < bytes.len() {
        let c0 = bytes[i];
        let c1 = bytes[i + 1];
        let c2 = bytes[i + 2];
        let c3 = bytes[i + 3];

        let v0 = b64_value(c0).ok_or("Invalid base64 character")?;
        let v1 = b64_value(c1).ok_or("Invalid base64 character")?;

        let v2 = if c2 == b'=' {
            0
        } else {
            b64_value(c2).ok_or("Invalid base64 character")?
        };

        let v3 = if c3 == b'=' {
            0
        } else {
            b64_value(c3).ok_or("Invalid base64 character")?
        };

        let triple = ((v0 as u32) << 18) | ((v1 as u32) << 12) | ((v2 as u32) << 6) | (v3 as u32);

        out.push(((triple >> 16) & 0xFF) as u8);

        if c2 != b'=' {
            out.push(((triple >> 8) & 0xFF) as u8);
        }

        if c3 != b'=' {
            out.push((triple & 0xFF) as u8);
        }

        i += 4;
    }

    Ok(BigUint::from_bytes_be(&out))
}
