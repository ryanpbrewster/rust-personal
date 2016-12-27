extern crate project_euler;
use project_euler::problems::problem_016;

extern crate num;
use self::num::bigint::BigUint;
use self::num::pow::pow;

#[test]
fn small() {
    assert_eq!(problem_016::solve(BigUint::from(123456u32)),
               1 + 2 + 3 + 4 + 5 + 6);
}

#[test]
fn main() {
    let n = pow(BigUint::from(2u32), 1000); // 2^1000
    assert_eq!(problem_016::solve(n), 1366);
}
