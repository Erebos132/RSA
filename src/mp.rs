// This file is for Message Parsing

use crate::gf;
use crate::kg;
use num_bigint::BigUint;

pub struct Msg {
    content: String,
}

impl Msg {
    pub fn new(message_string: &str) -> Msg {
        Msg {
            content: String::from(message_string),
        }
    }

    pub fn slice(&self, blocksize: usize) -> Vec<String> {
        let mut current_block_size = 0;
        let mut output_vect = vec![];
        let mut block = String::new();
        for char in self.content.chars() {
            current_block_size += 1;
            block.push(char);
            if current_block_size >= blocksize {
                current_block_size = 0;
                output_vect.push(block);
                block = String::new();
            }
        }
        if current_block_size != 0 {
            for _ in 0..=current_block_size {
                block.push(' ');
            }

            output_vect.push(block);
        }
        return output_vect;
    }

    pub fn encrypt(&self, public_keys: &(BigUint, BigUint)) -> Vec<BigUint> {
        let mut output_vect = vec![];

        for char in self.content.chars() {
            output_vect.push(kg::encrypt_num_for(&gf::big(char as u128), public_keys))
        }
        return output_vect;
    }

    pub fn decrypt(encrypted_message: Vec<BigUint>, key_identity: &kg::Keypair) -> String {
        let mut output_string = String::new();

        for encrypted_char in encrypted_message {
            output_string
                .push(key_identity.decrypt_num(&encrypted_char).to_u32_digits()[0] as u8 as char);
            println!("{output_string}");
        }

        return output_string;
    }
}
