/*
Find the last 10 digits of 28433 * 2^7830457 + 1
*/

use crate::util::math;
use num::bigint::BigUint;
use num::Integer;

pub fn solve() -> BigUint {
    let m: BigUint = BigUint::from(10_000_000_000u64);
    let n: BigUint = BigUint::from(28433u64)
        * math::powmod(&BigUint::from(2u64), &BigUint::from(7830457u64), &m)
        + BigUint::from(1u64);
    n.mod_floor(&m)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main() {
        assert_eq!(solve().to_str_radix(10), "8739992577");
    }
}
