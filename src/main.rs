#![allow(unused)]

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::rngs::OsRng;
use std::env::args;
use std::thread;
use std::time;

use crate::visualize::create_graph_stdev;

pub mod attacks;
pub mod gf;
pub mod kg;
pub mod mp;
pub mod padding;
pub mod rngp;
pub mod visualize;

fn main() {
    let arguments = args().collect::<Vec<String>>();
    create_graph_stdev(
        |num| {
            kg::Keypair::new(num as u64);
        },
        10,
        8,
        16,
        5,
        &arguments[1],
    );
}
