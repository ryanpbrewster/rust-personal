extern crate num;

use self::num::bigint::BigUint;
use self::num::pow::pow;

#[test]
fn small() {
    assert_eq!(solve(BigUint::from(123456u32)),
               1 + 2 + 3 + 4 + 5 + 6);
}

#[test]
fn main() {
    let n = pow(BigUint::from(2u32), 1000); // 2^1000
    assert_eq!(solve(n), 1366);
}

pub fn solve(n: BigUint) -> u32 {
    n.to_str_radix(10).chars().map(|ch| ch.to_digit(10).unwrap()).sum()
}
