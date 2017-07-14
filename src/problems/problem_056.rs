/*
A googol (10100) is a massive number: one followed by one-hundred zeros; 100100
is almost unimaginably large: one followed by two-hundred zeros. Despite their
size, the sum of the digits in each number is only 1.

Considering natural numbers of the form, ab, where a, b < 100, what is the
maximum digital sum?
*/

use num::pow;
use util::iter::Cross;
use num::BigUint;

pub fn solve(base_bound: u32, exponent_bound: u32) -> u32 {
    Cross::of(0..base_bound, 0..exponent_bound)
        .map(|(a, b)| digit_sum(pow(BigUint::from(a), b as usize)))
        .max()
        .expect("expect base_bound and exponent_bound to be non-zero")
}

fn digit_sum(n: BigUint) -> u32 {
    n.to_str_radix(10)
        .chars()
        .map(|ch| (ch as u32) - ('0' as u32))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main() {
        assert_eq!(solve(100, 100), 972);
    }
}
