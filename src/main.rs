#![allow(unused)]

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::rngs::OsRng;

pub mod gf;
pub mod kg;
pub mod mp;
pub mod rngp;

fn main() {
    let alice = kg::Keypair::new(128);
    let bob = kg::Keypair::new(128);
    let message = gf::big(123);

    let encrypt = bob.encrypt_num_for(&message, alice.get_public());
    println!("{encrypt}");

    println!("{}", alice.decrypt_num(&encrypt));
}
