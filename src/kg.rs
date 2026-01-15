use num_bigint::BigUint;

use crate::gf;
use crate::gf::big;
use crate::rngp;

#[derive(Debug)]
pub struct Keypair {
    n: BigUint,
    e: BigUint,
    d: BigUint,
}

impl Keypair {
    pub fn new(bitlength: u64) -> Keypair {
        let mut rng = rand::thread_rng();
        let p = rngp::get_prime_in_bitrange(&mut rng, bitlength, 64);
        let q = rngp::get_prime_in_bitrange(&mut rng, bitlength, 64);

        let n = &p * &q;
        let phi_n = (p - big(1)) * (q - big(1));

        let e = rngp::get_prime_in_bitrange(&mut rng, bitlength, 64);
        let d = gf::mod_inverse(&e, &phi_n);

        Keypair { n, e, d }
    }
}
