use crate::gf::big;
use num_bigint::BigUint;

pub fn factor(num: &BigUint) -> Vec<BigUint> {
    let mut testing_pointer = big(2);
    let mut factors = vec![];

    while (testing_pointer < num.sqrt()) {
        if (num % &testing_pointer == big(0)) {
            factors.push(testing_pointer.clone());
            factors.push(num / &testing_pointer);
        }
        testing_pointer += big(1);
    }

    return factors;
}

pub fn factor_n(num: &BigUint) -> Vec<BigUint> {
    let ballpark = num.sqrt() * big(2);
    let mut testing_pointer = &ballpark / big(2);

    while (testing_pointer < ballpark) {
        if (num % &testing_pointer == big(0)) {
            return vec![num / &testing_pointer, testing_pointer];
        }
        testing_pointer += big(1);
    }

    return vec![];
}
