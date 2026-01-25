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
    let mut rng = OsRng;

    println!(
        "{:?}",
        visualize::timer::timing_stdev(
            || {
                kg::Keypair::new(512);
            },
            200
        )
    );
}
