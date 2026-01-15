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

    println!("{:?}", kg::Keypair::new(256));
}
