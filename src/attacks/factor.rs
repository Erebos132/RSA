use crate::gf::big;
use num_bigint::BigUint;

pub fn factor(num: BigUint) -> Vec<BigUint> {
    let mut test = big(2);
    let mut factors = vec![];
    while &test < &(&num / &big(2) + &big(1)) {
        if &num % &test == big(0) {
            factors.push(test.clone());
        }
        test += big(1);
    }
    return factors;
}
