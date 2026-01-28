#![allow(unused)]

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::rngs::OsRng;
use std::env::args;
use std::thread;
use std::time;

use crate::visualize::create_graph_stdev_threaded;

pub mod attacks;
pub mod gf;
pub mod kg;
pub mod mp;
pub mod padding;
pub mod rngp;
pub mod visualize;

fn main() {
    let arguments = args().collect::<Vec<String>>();
    // create_graph_stdev_threaded(
    //     |num| {
    //         kg::Keypair::new(num as u64);
    //     },
    //     128,
    //     8,
    //     16,
    //     64,
    //     &arguments[1],
    // );
    let bob = kg::Keypair::new(1024);
    let message = mp::Msg::new("This will be a very long message with like a couple of sentences");

    println!("Found Key");

    println!(
        "{:?}",
        visualize::timer::timing_stdev(
            || {
                message.encrypt(bob.get_public());
            },
            200
        )
    );
}
