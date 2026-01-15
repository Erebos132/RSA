use num_bigint::{BigUint, RandBigInt, ToBigInt, ToBigUint};
use num_traits::cast::FromPrimitive;

pub fn big(num: u128) -> BigUint {
    return BigUint::from_u128(num).unwrap();
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

pub fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    let mut a_clone = a.clone();
    let mut b_clone = b.clone();
    if a_clone == b_clone {
        return a_clone;
    }
    if b_clone > a_clone {
        let temp = a_clone.clone();
        a_clone = b_clone.clone();
        b_clone = temp;
    }
    while &b_clone > &big(0) {
        let temp = a_clone.clone();
        a_clone = b_clone.clone();
        b_clone = temp % b_clone;
    }
    return a_clone;
}

pub fn mod_inv(a_u: &BigUint, m_u: &BigUint) -> BigUint {
    let a = a_u.to_bigint().unwrap();
    let m = m_u.to_bigint().unwrap();
    let (mut t, mut new_t) = (0.to_bigint().unwrap(), 1.to_bigint().unwrap());
    let (mut r, mut new_r) = (m.clone(), a % &m);

    while &new_r != &0.to_bigint().unwrap() {
        let quotient = (&r).clone() / &new_r;

        let temp_t = t;
        t = new_t.clone();
        new_t = temp_t - &quotient * &new_t;

        let temp_r = r;
        r = new_r.clone();
        new_r = temp_r - quotient * new_r;
    }

    if r > 1.to_bigint().unwrap() {
        // a and m are not coprime, inverse doesn't exist
        return big(0);
    }

    if t < 0.to_bigint().unwrap() {
        t += m;
    }

    t.to_biguint().unwrap()
}

pub fn mod_inverse(n: &BigUint, p: &BigUint) -> BigUint {
    if p <= &big(1) || gcd(n, p) > big(1) {
        return big(0);
    }
    return pmod(&n, &(p - &big(2)), &p);
}
