use num_bigint::BigUint;

pub fn gen_signature_for_hash(hash: BigUint, e: BigUint) -> BigUint {
    return hash.nth_root(e.to_u32_digits()[0]);
}
