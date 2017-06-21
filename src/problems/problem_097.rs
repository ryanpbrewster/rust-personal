/*
Find the last 10 digits of 28433 * 2^7830457 + 1
*/

use num::bigint::BigUint;
use num::{Integer, One};
use std::ops::Mul;

pub fn solve() -> BigUint {
    let m: BigUint = BigUint::from(10_000_000_000u64);
    let n: BigUint = BigUint::from(28433u64) * powmod(2u64, 7830457u64, &m) + BigUint::from(1u64);
    n.mod_floor(&m)
}

fn powmod(b: u64, e: u64, m: &BigUint) -> BigUint {
    if e == 0 {
        BigUint::one()
    } else {
        let mut t = powmod(b, e/2, m);
        t = (&t).mul(&t);
        if e % 2 == 1 {
            t = t.mul(BigUint::from(b));
        }
        t.mod_floor(m)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn powmod_works() {
        powmod(2u64, 42u64, &BigUint::from(1_000_000_000_000u64));
    }

    #[test]
    fn main() {
        assert_eq!(solve().to_str_radix(10), "8739992577");
    }
}
