// Collection of different padding algorithms, currently only random padding

// List of possible characters for padding (Base 64)
const CHARS: [char; 64] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z', '+', '/',
];

use kdam::format;
use rand::{self, Rng, seq::IteratorRandom};

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
pub fn add_oaep(n_bitlength: usize, message: &str) -> String {
    // Random Bits length
    let k0 = 256;
    let messg_bitlength = message.chars().count() * 8;
    let msg = message.as_bytes();
    let k1 = (n_bitlength - messg_bitlength - k0);

    return format!("{}", k1);
}
