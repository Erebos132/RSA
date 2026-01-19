// This File includes the logic for generating keys and with them encrypt / decrypt / sign / verify
// a simple number

#![allow(non_snake_case)]
use num_bigint::BigUint;

use std::thread;

use crate::gf::big;
use crate::mp::EncryptedMsg;
use crate::rngp;
use crate::{gf, mp};

// Keypair is the data storage unit for handeling keys (public + private)
#[derive(Debug)]
pub struct Keypair {
    public: (BigUint, BigUint),
    private: BigUint,
}

// Functions of keypairs
impl Keypair {
    // Create a new Keypair with bitlength n -> returns a Keypair with a matching private and
    // public keys
    pub fn new(bitlength: u64) -> Keypair {
        let mut rng = rand::rngs::OsRng;

        let (p, q) = Keypair::gen_pq(bitlength);

        let n = &p * &q;
        let phi_n = (&p - &big(1)) * (&q - &big(1));

        let e =
            rngp::get_prime_in_bitrange(&mut rng, (bitlength as f64).log(2.0) as u64 * 2 + 5, 64);
        let d = match gf::mod_inv(&e, &phi_n) {
            Some(v) => v,
            None => {
                eprintln!("d cannot be calculated, since e, phi(n) are NOT reverseable.");
                return Keypair::new(bitlength);
            }
        };

        Keypair {
            public: (n, e),
            private: d,
        }
    }

    // Create a custom Keypair from given Values
    pub fn from(n: BigUint, e: BigUint, d: BigUint) -> Keypair {
        Keypair {
            public: (n, e),
            private: d,
        }
    }

    // Function used to generate two (not equal) random big primes p, q in a given bitrange
    pub fn gen_pq(bitlength: u64) -> (BigUint, BigUint) {
        let mut rng = rand::rngs::OsRng;

        let (p, q) = (
            rngp::get_prime_in_bitrange(&mut rng, bitlength, 64),
            rngp::get_prime_in_bitrange(&mut rng, bitlength, 64),
        );

        if p == q {
            eprintln!("pq is equal, recalculating p,q");
            return Keypair::gen_pq(bitlength);
        }
        return (p, q);
    }

    // Functions for de- and encryption (just for Integers)
    pub fn encrypt_num(&self, messg_num: &BigUint) -> BigUint {
        return gf::pmod(messg_num, &self.public.1, &self.public.0);
    }

    pub fn decrypt_num(&self, encrypted_num: &BigUint) -> BigUint {
        return gf::pmod(encrypted_num, &self.private, &self.public.0);
    }

    // Signing a number to create a signature number
    pub fn sign_num(&self, messg_num: &BigUint) -> BigUint {
        return gf::pmod(
            &gf::hash_bytes(&messg_num.to_bytes_be()),
            &self.private,
            &self.public.0,
        );
    }

    // Verify Signature Number
    pub fn verify_num(
        original_messg_num: &BigUint,
        signature: &BigUint,
        public_key_sender: &(BigUint, BigUint),
    ) -> bool {
        return gf::hash_bytes(&original_messg_num.to_bytes_be())
            == gf::pmod(signature, &public_key_sender.1, &public_key_sender.0);
    }

    // Function to access public keys from outside -> To encrypt a message for someone you don't
    // know the private key from
    pub fn get_public(&self) -> &(BigUint, BigUint) {
        return &self.public;
    }

    // Encrypt not for yourself, but rather for a specific public key pair
    pub fn encrypt_num_for(messg_num: &BigUint, public_keys_recv: &(BigUint, BigUint)) -> BigUint {
        return gf::pmod(messg_num, &public_keys_recv.1, &public_keys_recv.0);
    }

    // Encrypt a parsed message and return an encrypted message (different format for handling the
    // Blocks of encrypted nums and then to decrypt again)
    pub fn encrypt_msg_for(
        &self,
        msg: mp::Msg,
        public_keys_recv: &(BigUint, BigUint),
    ) -> EncryptedMsg {
        msg.encrypt(public_keys_recv)
    }

    // Function to retrieve a message encrypted for the matching public keys of oneselves' identity
    pub fn decrypt_msg(&self, encrypted_msg: mp::EncryptedMsg) -> String {
        encrypted_msg.decrypt(&self)
    }

    // Function to show the current key pair, useful for debugging mainly
    pub fn display(&self) -> String {
        format![
            "public keys: n={}; e={}\nprivate key: d={}",
            self.public.0, self.public.1, self.private
        ]
    }
}
