// This file is for testing what happens when you choose a low public exponent and can therefore
// recover the message from that
//
// m^e < n -> no encryption takes place

use crate::gf::big;
use crate::kg::Keypair;
use crate::rngp;
use crate::{gf, mp};
use num_bigint::BigUint;

pub fn test() {
    let bitlength = 512;
    let mut rng = rand::rngs::OsRng;

    let (p, q) = Keypair::gen_pq(bitlength);

    let n = &p * &q;
    let phi_n = (&p - &big(1)) * (&q - &big(1));

    let e = big(5);
    let d = match gf::mod_inv(&e, &phi_n) {
        Some(v) => v,
        None => {
            println!("d cannot be calculated, since e, phi(n) are NOT reverseable.");
            return test();
        }
    };

    let bob = Keypair::from(n, e.clone(), d);
    let message = mp::Msg::new("Hello, my name is bendix");
    let encrypted = message.encrypt_blocks(8, bob.get_public());
    println!("{:?}", encrypted.display());

    println!(
        "{}",
        gf::int_to_str(&gf::nth_root(
            &encrypted.display().get(0).unwrap(),
            *e.to_u32_digits().get(0).unwrap()
        ))
    );
}
