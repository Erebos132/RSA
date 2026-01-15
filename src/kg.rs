use num_bigint::BigUint;

use crate::gf;
use crate::gf::big;
use crate::rngp;

#[derive(Debug)]
pub struct Keypair {
    public: (BigUint, BigUint),
    private: BigUint,
}

impl Keypair {
    pub fn new(bitlength: u64) -> Keypair {
        let mut rng = rand::thread_rng();
        let p = rngp::get_prime_in_bitrange(&mut rng, bitlength, 64);
        let q = rngp::get_prime_in_bitrange(&mut rng, bitlength, 64);

        let n = &p * &q;
        let phi_n = (&p - &big(1)) * (&q - &big(1));

        let e = rngp::get_prime_in_bitrange(&mut rng, bitlength, 64);
        let d = gf::mod_inv(&e, &phi_n);

        Keypair {
            public: (n, e),
            private: d,
        }
    }

    pub fn encrypt_num(&self, messg_num: &BigUint) -> BigUint {
        return gf::pmod(messg_num, &self.public.1, &self.public.0);
    }

    pub fn decrypt_num(&self, encrypted_num: &BigUint) -> BigUint {
        return gf::pmod(encrypted_num, &self.private, &self.public.0);
    }
}
