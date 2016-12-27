extern crate num;
use self::num::bigint::BigUint;
use self::num::One;

use std::ops::Mul;

pub fn solve(n: u32) -> u32 {
    digit_sum(big_factorial(n))
}

fn big_factorial(n: u32) -> BigUint {
    (2..n+1).map(|i| BigUint::from(i)).fold(BigUint::one(), |a, b| a.mul(b))
}

fn digit_sum(b: BigUint) -> u32 {
    b.to_str_radix(10).chars().map(|c| c.to_digit(10).unwrap()).sum()
}
