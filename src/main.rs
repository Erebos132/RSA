#![allow(unused)]

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::rngs::OsRng;

pub mod attacks;
pub mod gf;
pub mod kg;
pub mod mp;
pub mod rngp;

fn main() {
    // let bob = kg::Keypair::new(256);
    println!("{:?}", attacks::factor::factor(gf::big(5122102)));
}
