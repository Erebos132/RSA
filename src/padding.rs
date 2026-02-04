// Collection of different padding algorithms, currently only random padding

// List of possible characters for padding (Base 64)
const CHARS: [char; 64] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z', '+', '/',
];

use kdam::format;
use num_bigint::BigUint;
use rand::{self, Rng, seq::IteratorRandom};

use crate::gf;
use crate::rngp;

// Generating a specific amount of random characters chosen from the possible characters
pub fn random_padding(num: usize) -> String {
    let mut output = String::new();
    let mut rng = rand::rngs::OsRng;

    for _ in 0..num {
        output.push(*CHARS.iter().choose(&mut rng).unwrap());
    }
    return output;
}

// Return a new message containing the original message surrounded by the padding with length
// "length"
pub fn add_random_padding(message: &str, length: usize) -> String {
    return format!(
        "{}{}{}",
        random_padding(length),
        message,
        random_padding(length)
    );
}

// Remove the "length" amount of outer characters
pub fn remove_padding(message: &str, length: usize) -> String {
    String::from(&message[length..message.len() - length])
}

// Function for OAEP Implementation
pub fn add_oaep(n_bitlength: usize, message: &str) -> (BigUint, BigUint) {
    // Random Bits length
    let k0 = 256;
    // Generate random bits:
    let mut rng = rand::rngs::OsRng;
    let r = &rngp::gen_n_random_bits(&mut rng, k0)[..];

    let g_of_r = gf::hash_bytes_col(r, n_bitlength - k0);

    let messg_bitlength = message.chars().count() * 8;
    let k1 = (n_bitlength - messg_bitlength - k0);
    let msg = message.as_bytes();

    let xor1 = gf::xor(msg, &g_of_r[..]);

    let h_of_x = gf::hash_bytes_col(&xor1[..], k0);
    let xor2 = gf::xor(r, &h_of_x[..]);

    return (
        BigUint::from_bytes_be(&xor1[..]),
        BigUint::from_bytes_be(&xor2[..]),
    );
}

pub fn remove_oaep(n_bitlength: usize, (x, y): (BigUint, BigUint)) -> String {
    let (x, y) = (x.to_bytes_be(), y.to_bytes_be());
    // Random Bits length
    let k0 = 256;

    let h_of_x = gf::hash_bytes_col(&x, k0);
    let r = gf::xor(&y, &h_of_x[..]);

    let g_of_r = gf::hash_bytes_col(&r, n_bitlength - k0);

    let msg = gf::xor(&x, &g_of_r);

    return String::from_utf8(msg).unwrap();
}
