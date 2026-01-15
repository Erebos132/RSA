#![allow(unused)]

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::thread_rng;

pub mod gf;
pub mod kg;
pub mod rngp;

fn main() {
    let mut rng = thread_rng();
    // println!("{}", rngp::get_prime_in_bitrange(&mut rng, 256, 64));
    // println!("{}", rngp::get_prime_in_bitrange(&mut rng, 256, 64));

    let keypair = kg::Keypair::new(128);
    let message = gf::big(500012);
    let encrypt_messg = keypair.encrypt_num(&message);
    let decrypt_messg = keypair.decrypt_num(&encrypt_messg);
    println!("{} = {}", message, decrypt_messg);
}
