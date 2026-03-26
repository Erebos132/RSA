#![allow(unused)]

use dialoguer;
use dialoguer::theme::ColorfulTheme;
use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::env::args;
use std::thread;

use crate::attacks::{chosen_plaintext, factor};

pub mod attacks;
pub mod gf;
pub mod kg;
pub mod mp;
pub mod padding;
pub mod rngp;
pub mod visualize;

fn main() {
    let arguments = args().collect::<Vec<String>>();
    let theme = ColorfulTheme::default();

    let mut keyrings: HashMap<String, kg::Keypair> = HashMap::new();

    loop {
        let basic_categories = dialoguer::Select::with_theme(&theme)
            .with_prompt("What do you want to do?")
            .items(&[
                "Encrypt Message",
                "Decrypt Message",
                "Generate Keypair",
                "Attack",
                "Quit",
            ])
            .default(2)
            .interact()
            .unwrap();

        // Encryption
        if basic_categories == 0 {
            let msg: String = dialoguer::Input::with_theme(&theme)
                .with_prompt("What Message should be Encrypted?")
                .interact_text()
                .unwrap();
            let keyring_names = keyrings.keys().cloned().collect::<Vec<String>>();
            let index = dialoguer::Select::with_theme(&theme)
                .with_prompt("Select Recipient")
                .items(&keyring_names)
                .default(0)
                .interact()
                .unwrap();

            let selected_name = &keyring_names[index];

            let recv: &kg::Keypair = keyrings.get(selected_name.as_str()).unwrap();

            let padding = dialoguer::Select::with_theme(&theme)
                .with_prompt("Should the Message be Padded?")
                .items(&["No Padding", "OAEP", "Random Characters"])
                .default(0)
                .interact()
                .unwrap();

            let blocksize = dialoguer::Select::with_theme(&theme)
                .with_prompt("Choose Blocksize:")
                .items((1..=10).map(|intbef| intbef.to_string()))
                .default(4 - 1)
                .interact()
                .unwrap()
                + 1;

            let message = match padding {
                0 => mp::Msg::new(&msg).encrypt_blocks(blocksize, recv.get_public()),
                1 => {
                    let keysize = selected_name
                        .split(":")
                        .collect::<Vec<&str>>()
                        .pop()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    mp::Msg::new(&msg).encrypt_oaep(blocksize, recv.get_public(), keysize)
                }
                2 => {
                    let padding_size = dialoguer::Select::with_theme(&theme)
                        .with_prompt("Choose Size of Padding (for each block):")
                        .items((1..=10).map(|intbef| intbef.to_string()))
                        .default(2 - 1)
                        .interact()
                        .unwrap();
                    mp::Msg::new(&msg).encrypt_blocks_padding(
                        blocksize,
                        padding_size,
                        recv.get_public(),
                    )
                }

                _ => mp::EncryptedMsg::new(vec![]),
            };

            println!("======== Encrypted Message ==========");
            println!("{}", message.base64());
            println!("=====================================");
        }

        // Decryption
        if basic_categories == 1 {
            let msg_b64: String = dialoguer::Input::with_theme(&theme)
                .with_prompt("Paste Encrypted Message")
                .interact_text()
                .unwrap();
            let msg: mp::EncryptedMsg = mp::EncryptedMsg::from_base64(&msg_b64);

            let keyring_names = keyrings.keys().cloned().collect::<Vec<String>>();
            let index = dialoguer::Select::with_theme(&theme)
                .with_prompt("Who are you? ")
                .items(&keyring_names)
                .default(0)
                .interact()
                .unwrap();

            let selected_name = &keyring_names[index];

            let iden: &kg::Keypair = keyrings.get(selected_name.as_str()).unwrap();

            let padding = dialoguer::Select::with_theme(&theme)
                .with_prompt("How was the Message Padded?")
                .items(&["No Padding", "OAEP", "Random Characters"])
                .default(0)
                .interact()
                .unwrap();

            let message = match padding {
                0 => msg.decrypt_blocks(&iden),
                1 => {
                    let keysize = selected_name
                        .split(":")
                        .collect::<Vec<&str>>()
                        .pop()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    msg.decrypt_oaep(&iden, keysize)
                }
                2 => {
                    let padding_size = dialoguer::Select::with_theme(&theme)
                        .with_prompt("How many Characters of Padding were used? (for each block)")
                        .items((1..=10).map(|intbef| intbef.to_string()))
                        .default(2 - 1)
                        .interact()
                        .unwrap();
                    msg.decrypt_blocks_padding(iden, padding_size)
                }

                _ => String::new(),
            };

            println!("======== Decrypted Message ==========");
            println!("{}", message);
            println!("=====================================");
        }

        // Keygeneration
        if basic_categories == 2 {
            let keypair_name: String = dialoguer::Input::with_theme(&theme)
                .with_prompt("Name:")
                .interact_text()
                .unwrap();
            let keysizes = vec![12, 16, 20, 32, 64, 128, 256, 512, 1024, 2048, 4096];
            let factor_size = dialoguer::Select::with_theme(&theme)
                .with_prompt("Select Size of Prime Numbers (Factors for Modulus)")
                .items(&keysizes)
                .default(6)
                .interact()
                .unwrap();

            if dialoguer::Confirm::with_theme(&theme)
                .default(true)
                .with_prompt("Are you sure?")
                .interact()
                .unwrap()
            {
                keyrings.insert(
                    format!("{}:{}", keypair_name, keysizes[factor_size]),
                    kg::Keypair::new(keysizes[factor_size]),
                );
            }
        }

        // Attacks
        if basic_categories == 3 {
            match dialoguer::Select::with_theme(&theme)
                .with_prompt("Which Attack do you want to execute")
                .items(&[
                    "Modulus Factorization",
                    "Chosen Plaintext",
                    "Chosen Cyphertext",
                ])
                .default(0)
                .interact()
                .unwrap()
            {
                0 => {
                    // Modulus Factorization
                    let keyring_names = keyrings.keys().cloned().collect::<Vec<String>>();
                    let index = dialoguer::Select::with_theme(&theme)
                        .with_prompt("Select Identity to Crack")
                        .items(&keyring_names)
                        .default(0)
                        .interact()
                        .unwrap();

                    let selected_name = &keyring_names[index];

                    let iden: &kg::Keypair = keyrings.get(selected_name.as_str()).unwrap();
                    let factors = factor::factor(&iden.get_public().0);

                    println!("Found Modulus Factors: {} and {}", factors[0], factors[1]);
                }
                1 => {
                    // Chosen Plaintext
                    let msg_b64: String = dialoguer::Input::with_theme(&theme)
                        .with_prompt("Paste Encrypted Message")
                        .interact_text()
                        .unwrap();
                    let msg: mp::EncryptedMsg = mp::EncryptedMsg::from_base64(&msg_b64);

                    let keyring_names = keyrings.keys().cloned().collect::<Vec<String>>();
                    let index = dialoguer::Select::with_theme(&theme)
                        .with_prompt("Who was this Message meant to be sent to?")
                        .items(&keyring_names)
                        .default(0)
                        .interact()
                        .unwrap();

                    let selected_name = &keyring_names[index];

                    let iden: &kg::Keypair = keyrings.get(selected_name.as_str()).unwrap();

                    let blocksize = dialoguer::Select::with_theme(&theme)
                        .with_prompt("Which Blocksize was used?")
                        .items((1..=10).map(|intbef| intbef.to_string()))
                        .default(4 - 1)
                        .interact()
                        .unwrap()
                        + 1;

                    println!(
                        "Found Message: {}",
                        chosen_plaintext::unknown_message_length(
                            &msg,
                            iden.get_public(),
                            blocksize
                        )
                    );
                }
                _ => (),
            };
        }

        // Exit
        if basic_categories == 4 {
            std::process::exit(0)
        }
    }
}
