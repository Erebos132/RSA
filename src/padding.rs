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
// WORKS ONLY FOR BITLENGTH >= 512 bits!
pub fn add_oaep(n_bitlength: usize, message: &str) -> BigUint {
    // Random Bits length
    let mut k0 = 256;

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

    return BigUint::from_bytes_be(&oaep_encode(xor1, xor2, k0));
}

fn oaep_encode(x: Vec<u8>, y: Vec<u8>, n: usize) -> Vec<u8> {
    let mut em = Vec::with_capacity(n + 1);

    em.push(0x01);
    em.extend_from_slice(&x[..]);
    em.extend_from_slice(&y[..]);

    em
}

fn oaep_decode(em: &[u8], k0: usize) -> (Vec<u8>, Vec<u8>) {
    // EM = 0x00 || X || Y

    if em[0] != 0x01 {
        panic!("Invalid OAEP encoding");
    }

    let x_len = em.len() - 1 - k0 / 8;

    let x = em[1..1 + x_len].to_vec();
    let y = em[1 + x_len..].to_vec();

    (x, y)
}

pub fn remove_oaep(n_bitlength: usize, em: &BigUint) -> String {
    let (x, y) = oaep_decode(&em.to_bytes_be(), 256);

    // Random Bits length
    let k0 = 256;

    let h_of_x = gf::hash_bytes_col(&x, k0);
    let r = gf::xor(&y, &h_of_x[..]);

    let g_of_r = gf::hash_bytes_col(&r, n_bitlength - k0);

    let msg = gf::xor(&x, &g_of_r);

    return String::from_utf8(msg).unwrap();
}
