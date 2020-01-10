use num::BigUint;
use std::cmp::{Ord, Ordering};
use std::collections::BinaryHeap;

pub fn solve(n: usize) -> BigUint {
    let mut count = 0;

    let mut pq: BinaryHeap<DigitSumPower> = BinaryHeap::new();
    pq.push(DigitSumPower::new(2));

    let mut cur = 2;
    while let Some(mut dsp) = pq.pop() {
        let bound = max_possible_digit_sum(&dsp.n);
        while cur < bound {
            cur += 1;
            pq.push(DigitSumPower::new(cur));
        }
        if dsp.power > 1 && digit_sum(&dsp.n) == dsp.base {
            count += 1;
            if count == n {
                return dsp.n;
            }
        }
        dsp.increment();
        pq.push(dsp);
    }
    panic!("iteration is unbounded, control should not reach here");
}

fn max_possible_digit_sum(n: &BigUint) -> u32 {
    let num_digits = n.to_str_radix(10).len();
    9 * num_digits as u32
}

fn digit_sum(n: &BigUint) -> u32 {
    n.to_str_radix(10)
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .sum()
}

#[derive(Eq, Ord, Clone, Debug)]
struct DigitSumPower {
    base: u32,
    power: u32,
    n: BigUint,
}

impl PartialEq for DigitSumPower {
    fn eq(&self, other: &DigitSumPower) -> bool {
        self.n == other.n
    }
}
impl PartialOrd for DigitSumPower {
    fn partial_cmp(&self, other: &DigitSumPower) -> Option<Ordering> {
        Some(BigUint::cmp(&self.n, &other.n).reverse())
    }
}

impl DigitSumPower {
    fn new(base: u32) -> DigitSumPower {
        DigitSumPower {
            base: base,
            power: 1,
            n: BigUint::from(base),
        }
    }
    fn increment(&mut self) {
        self.power += 1;
        self.n = &self.n * BigUint::from(self.base);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn small() {
        assert_eq!(solve(2), BigUint::from(512u32));
        assert_eq!(solve(10), BigUint::from(614_656u32));
    }

    #[test]
    fn main() {
        assert_eq!(solve(30), BigUint::from_str("248155780267521").unwrap());
    }
}
