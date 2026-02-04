// This file is for Message Parsing

use crate::gf;
use crate::kg;
use crate::padding;
use num_bigint::BigUint;

pub struct EncryptedMsg {
    blocks: Vec<BigUint>,
}

// Format for storing encrypted nums as blocks
impl EncryptedMsg {
    pub fn new(blocks: Vec<BigUint>) -> EncryptedMsg {
        EncryptedMsg { blocks }
    }

    // Retrieve (potentially part of) original message by decrypting the first block stored
    pub fn decrypt(&self, key_identity: &kg::Keypair) -> String {
        let mut output_string = String::new();

        for encrypted_char in &self.blocks {
            output_string
                .push(key_identity.decrypt_num(&encrypted_char).to_u32_digits()[0] as u8 as char);
        }

        return output_string;
    }

    // Decrypt all blocks and put them together to form the original message
    pub fn decrypt_blocks(&self, key_identity: &kg::Keypair) -> String {
        let mut output_string = String::new();

        for block in &self.blocks {
            output_string += &(gf::int_to_str(&key_identity.decrypt_num(&block)));
        }

        return output_string;
    }

    // Decrypt all blocks stored and but them together after padding around the message was removed
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

    // TODO: This does not work yet, by no means, not even close...
    // pub fn verify(&self, public_sender: (BigUint, BigUint), orig_msg: Msg) -> bool {
    //     kg::Keypair::verify_num(original_messg_num, signature, public_key_sender)
    // }

    // Get the inner blocks which are encrypted
    pub fn display(&self) -> &Vec<BigUint> {
        return &self.blocks;
    }
}

// Normal Struct for creating messages
pub struct Msg {
    content: String,
}

impl Msg {
    pub fn new(message_string: &str) -> Msg {
        Msg {
            content: String::from(message_string),
        }
    }

    // Turn the Message content into blocks of a fixed size and return the list of blocks, each
    // block being a substring of the original message; if the block is not filled completely by
    // the message, it is filled with spaces (placeholders)
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

    // Create an encrypted message by encrypting the message as one big block
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

    // Encrypt the message by blockizing it first, then encrypting every block and putting them in
    // the data format of an encrypted message.
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

    // Encrypt message in blocks (see above), but also add a specific amount of padding around each
    // block (random characters) (Padding could be changed later)
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

    // TODO: This function is not done yet
    pub fn encrypt_oaep(&self, blocksize: usize, public_keys: &(BigUint, BigUint)) -> EncryptedMsg {
        let mut output_vec = vec![];
        return EncryptedMsg::new(output_vec);
    }

    pub fn sign(&self, keyring: &kg::Keypair) -> EncryptedMsg {
        let mut output_vect = vec![];

        for char in self.content.chars() {
            output_vect.push(keyring.sign_num(&gf::big(char as u128)));
        }
        return EncryptedMsg::new(output_vect);
    }
}
