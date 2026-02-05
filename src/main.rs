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
    let bob = kg::Keypair::new(64);

    println!("{}", bob.get_public().0);
    println!("{}", gf::base64_encode(&bob.get_public().0));
    println!(
        "{}",
        gf::base64_decode(gf::base64_encode(&bob.get_public().0)).unwrap()
    );

    // println!(
    //     "{:?}",
    //     mp::Msg::new("abc das ist ein Test!")
    //         .encrypt_oaep(5, bob.get_public(), 512)
    //         .decrypt_oaep(&bob, 512)
    // );
}
