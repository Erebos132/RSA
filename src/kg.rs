// This File includes the logic for generating keys and with them encrypt / decrypt / sign / verify
// a simple number

#![allow(non_snake_case)]
use num_bigint::BigUint;

use std::thread;

use crate::gf::big;
use crate::mp::EncryptedMsg;
use crate::rngp;
use crate::{gf, mp};

#[derive(Debug)]
pub struct Keypair {
    public: (BigUint, BigUint),
    private: BigUint,
}

impl Keypair {
    pub fn new(bitlength: u64) -> Keypair {
        // Multithreading, not that much more efficient, implement when too much time left
        // let p_handle = thread::spawn(move || {
        //     let mut rng = rand::rngs::OsRng;
        //     rngp::get_prime_in_bitrange(&mut rng, bitlength, 64)
        // });
        //
        // let q_handle = thread::spawn(move || {
        //     let mut rng = rand::rngs::OsRng;
        //     rngp::get_prime_in_bitrange(&mut rng, bitlength, 64)
        // });
        //
        // let p = p_handle.join().unwrap();
        // let q = q_handle.join().unwrap();

        let mut rng = rand::rngs::OsRng;

        let (p, q) = (
            rngp::get_prime_in_bitrange(&mut rng, bitlength, 64),
            rngp::get_prime_in_bitrange(&mut rng, bitlength, 64),
        );

        if p == q {
            eprintln!("pq is equal, recalculating p,q");
            return Keypair::new(bitlength);
        }

        let n = &p * &q;
        let phi_n = (&p - &big(1)) * (&q - &big(1));

        let e =
            rngp::get_prime_in_bitrange(&mut rng, (bitlength as f64).log(2.0) as u64 * 2 + 5, 64);
        let d = match gf::mod_inv(&e, &phi_n) {
            Some(v) => v,
            None => panic!["The Key Pair is not inversible!"],
        };

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

    pub fn sign(&self, messg_num: &BigUint) -> BigUint {
        return gf::pmod(&gf::hash(messg_num), &self.private, &self.public.0);
    }

    pub fn verify(
        &self,
        original_messg_num: &BigUint,
        signature: &BigUint,
        public_key_sender: &(BigUint, BigUint),
    ) -> bool {
        return gf::hash(original_messg_num)
            == gf::pmod(signature, &public_key_sender.1, &public_key_sender.0);
    }

    pub fn get_public(&self) -> &(BigUint, BigUint) {
        return &self.public;
    }

    pub fn encrypt_num_for(messg_num: &BigUint, public_keys_recv: &(BigUint, BigUint)) -> BigUint {
        return gf::pmod(messg_num, &public_keys_recv.1, &public_keys_recv.0);
    }

    pub fn encrypt_msg_for(
        &self,
        msg: mp::Msg,
        public_keys_recv: &(BigUint, BigUint),
    ) -> EncryptedMsg {
        msg.encrypt(public_keys_recv)
    }

    pub fn decrypt_msg(&self, encrypted_msg: mp::EncryptedMsg) -> String {
        encrypted_msg.decrypt(&self)
    }

    pub fn display(&self) -> String {
        format![
            "public keys: n={}; e={}\nprivate key: d={}",
            self.public.0, self.public.1, self.private
        ]
    }
}
