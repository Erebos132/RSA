#![allow(unused)]

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::rng;

pub mod gf;
pub mod rngp;

fn main() {
    let mut rng = rng();
    let number = 6.to_biguint().unwrap();
    println!("{}", rngp::check_prime(&number, 10));
}
