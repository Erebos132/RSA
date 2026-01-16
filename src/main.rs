#![allow(unused)]

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::rngs::OsRng;
use std::thread;
use std::time;

pub mod attacks;
pub mod gf;
pub mod kg;
pub mod mp;
pub mod rngp;

fn main() {
    // let now = time::Instant::now();
    // let ratio = 16;
    // let average = 10000;
    //
    // for _ in 0..average / ratio {
    //     let mut threads = vec![];
    //     for _ in 0..ratio {
    //         threads.push(thread::spawn(|| kg::Keypair::new(32)));
    //     }
    //
    //     for thread in threads {
    //         thread.join().unwrap().display();
    //     }
    // }
    // println!("{:?}", now.elapsed() / average);
    //
    let bob = kg::Keypair::new(24);
    println!("{:?}", attacks::factor::factor(&bob.get_public().0));
}
