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
    let (p, q) = kg::Keypair::gen_pq(16);
    let bob = kg::Keypair::from_pqe(p, q, gf::big(3));

    let message = mp::Msg::new("Testing, Hello World");
    let signed_message = message.sign(&bob.unwrap());
    println!("{:?}", signed_message.display());

    // attacks::signature_forgery::gen_signature_for_hash(hash, e);
}
