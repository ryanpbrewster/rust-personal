extern crate num;
use self::num::bigint::BigInt;
use self::num::traits::Zero;

pub fn solve(numbers: Vec<BigInt>) -> BigInt {
    numbers.iter().fold(BigInt::zero(), |a, b| a + b)
}
