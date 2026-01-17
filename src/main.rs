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
pub mod rngp;
pub mod visualize;

fn main() {
    let arguments = args().collect::<Vec<String>>();
    visualize::create_graph(
        |bitlength| {
            kg::Keypair::new(bitlength as u64);
        },
        12,
        16,
        16,
        3,
        &arguments[1],
    );
}
