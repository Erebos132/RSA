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
    println!(
        "{:?}",
        visualize::timer::timing_st_dev(
            || {
                kg::Keypair::new(16);
            },
            500
        )
    );
    // attacks::low_pub::test();
}
