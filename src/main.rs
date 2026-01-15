#![allow(unused)]

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::thread_rng;

pub mod gf;
pub mod rngp;

fn main() {
    let mut rng = thread_rng();
    let number = gf::big(6890677128309814912091);
    println!("{}", rngp::get_prime_in_bitrange(&mut rng, 100, 10));
}
