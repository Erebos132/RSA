// This file is for Message Parsing

use crate::gf;
use crate::kg;
use crate::padding;
use num_bigint::BigUint;

pub struct EncryptedMsg {
    blocks: Vec<BigUint>,
}

impl EncryptedMsg {
    pub fn new(blocks: Vec<BigUint>) -> EncryptedMsg {
        EncryptedMsg { blocks }
    }

    pub fn decrypt(&self, key_identity: &kg::Keypair) -> String {
        let mut output_string = String::new();

        for encrypted_char in &self.blocks {
            output_string
                .push(key_identity.decrypt_num(&encrypted_char).to_u32_digits()[0] as u8 as char);
        }

        return output_string;
    }

    pub fn decrypt_blocks(&self, key_identity: &kg::Keypair) -> String {
        let mut output_string = String::new();

        for block in &self.blocks {
            output_string += &(gf::int_to_str(&key_identity.decrypt_num(&block)));
        }

        return output_string;
    }

    pub fn decrypt_blocks_padding(
        &self,
        key_identity: &kg::Keypair,
        padding_size: usize,
    ) -> String {
        let mut output_string = String::new();

        for block in &self.blocks {
            output_string += &(padding::remove_padding(
                &gf::int_to_str(&key_identity.decrypt_num(&block)),
                padding_size,
            ));
        }

        return output_string;
    }

    pub fn display(&self) -> &Vec<BigUint> {
        return &self.blocks;
    }
}

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

    pub fn encrypt(&self, public_keys: &(BigUint, BigUint)) -> EncryptedMsg {
        let mut output_vect = vec![];

        for char in self.content.chars() {
            output_vect.push(kg::Keypair::encrypt_num_for(
                &gf::big(char as u128),
                public_keys,
            ))
        }
        return EncryptedMsg::new(output_vect);
    }

    pub fn encrypt_blocks(
        &self,
        blocksize: usize,
        public_keys: &(BigUint, BigUint),
    ) -> EncryptedMsg {
        let mut output_vect = vec![];

        for block in self.slice(blocksize) {
            output_vect.push(kg::Keypair::encrypt_num_for(
                &gf::str_to_int(&block),
                public_keys,
            ))
        }
        return EncryptedMsg::new(output_vect);
    }

    pub fn encrypt_blocks_padding(
        &self,
        blocksize: usize,
        padding_size: usize,
        public_keys: &(BigUint, BigUint),
    ) -> EncryptedMsg {
        let mut output_vect = vec![];

        for block in self.slice(blocksize) {
            output_vect.push(kg::Keypair::encrypt_num_for(
                &gf::str_to_int(&padding::add_random_padding(&block, padding_size)),
                public_keys,
            ))
        }
        return EncryptedMsg::new(output_vect);
    }
}
