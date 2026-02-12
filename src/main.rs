#![allow(unused)]

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::rngs::OsRng;
use std::env::args;
use std::thread;
use std::time;

pub mod attacks;
pub mod gf;
pub mod kg;
pub mod mp;
pub mod padding;
pub mod rngp;
pub mod visualize;

fn main() {
    let arguments = args().collect::<Vec<String>>();
    let bob = kg::Keypair::new(512);
    let message = "tes";
    let msg = mp::Msg::new(message).encrypt_blocks(message.len(), bob.get_public());

    let now = time::Instant::now();
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut round = 0;
    for _ in 0..100 {
        loop {
            round += 1;
            if round % 300000 == 0 {
                println!("round {round} completed after {:?}", now.elapsed());
            }
            let testing_msg = mp::Msg::new(&padding::generate_random(message.len(), &charset));
            if (testing_msg
                .encrypt_blocks(message.len(), bob.get_public())
                .display()
                == msg.display())
            {
                println!("found msg: {}", testing_msg.display());
                println!("took {:?}", now.elapsed());
                break;
            }
        }
    }
    println!("{:?}", now.elapsed() / 100);
}
