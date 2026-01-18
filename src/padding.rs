const CHARS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

use rand::{self, Rng, seq::IteratorRandom};
pub fn random_padding(num: usize) -> String {
    let mut output = String::new();
    let mut rng = rand::rngs::OsRng;

    for _ in 0..num {
        output.push(*CHARS.iter().choose(&mut rng).unwrap());
    }
    return output;
}

pub fn add_random_padding(message: &str, length: usize) -> String {
    return format!(
        "{}{}{}",
        random_padding(length),
        message,
        random_padding(length)
    );
}

pub fn remove_padding(message: &str, length: usize) -> String {
    String::from(&message[length..message.len() - length])
}
