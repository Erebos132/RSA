use num_bigint::{BigUint, RandBigInt, ToBigUint};

pub fn big(num: u128) -> BigUint {
    return num.to_biguint().unwrap();
}

// Function for quickly calculating base^power mod modulus
pub fn pmod(base: &BigUint, power: &BigUint, modulus: &BigUint) -> BigUint {
    let mut base_clone = base.clone();
    let mut power_clone = power.clone();

    let mut result = big(1);

    while &power_clone >= &big(1) {
        // Odd
        if &power_clone % big(2) == big(1) {
            result = (result * &base_clone) % modulus;
            power_clone -= big(1);
        }
        // Even
        else {
            base_clone = (&base_clone * &base_clone) % modulus;
            power_clone /= big(2);
        }
    }
    return result;
}
