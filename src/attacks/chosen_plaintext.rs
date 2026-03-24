use crate::kg::Keypair;
use crate::mp;
use crate::mp::EncryptedMsg;
use num_bigint::BigUint;

const ORDER: [char; 92] = [
    ' ', '!', '"', '#', '$', '%', '&', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3',
    '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F',
    'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
    'Z', '[', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}',
];

fn char_index(c: char) -> Option<usize> {
    ORDER.iter().position(|&x| x == c)
}

fn increment(msg: &str) -> String {
    let mut chars: Vec<char> = msg.chars().collect();

    // Start from the last character
    let mut i = chars.len();
    while i > 0 {
        i -= 1;

        let idx = char_index(chars[i]).expect("Invalid character in msg");

        if idx + 1 < ORDER.len() {
            // No carry needed
            chars[i] = ORDER[idx + 1];
            return chars.iter().collect();
        } else {
            // Carry over
            chars[i] = ORDER[0];
            // Continue to next left character
        }
    }

    // If we reached here, we carried past the first char → add a new one at the front
    let mut new_msg = vec![ORDER[0]];
    new_msg.extend(chars);
    new_msg.iter().collect()
}
pub fn unknown_message_length(
    msg: &EncryptedMsg,
    recv_pub: &(BigUint, BigUint),
    blocksize: usize,
) -> String {
    let mut final_message = String::new();
    let mut test_message = String::from(" ");
    for block in msg.display() {
        while block
            != &mp::Msg::new(&test_message)
                .encrypt_blocks(blocksize, recv_pub)
                .display()[0]
        {
            test_message = increment(&test_message);
        }
        println!("Found Block: {}", test_message);
        final_message += &test_message;
    }
    final_message
}
