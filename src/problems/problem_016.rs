extern crate num;

use self::num::bigint::BigUint;

pub fn solve(n: BigUint) -> u32 {
    n.to_str_radix(10).chars().map(|ch| ch.to_digit(10).unwrap()).sum()
}
