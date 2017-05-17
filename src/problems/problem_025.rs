extern crate num;
use self::num::bigint::BigUint;

use std::mem;

pub fn solve(num_digits: usize) -> usize {
    let threshold: BigUint = num::pow::pow(BigUint::from(10u32), num_digits - 1);
    BigFib::all().enumerate().find(|&(_, ref n)| n >= &threshold).map(|(idx, _)| idx).unwrap()
}

struct BigFib {
    cur: BigUint,
    prev: BigUint,
}

impl BigFib {
    fn all() -> BigFib {
        BigFib {
            cur: BigUint::from(1u32),
            prev: BigUint::from(0u32),
        }
    }
}

impl Iterator for BigFib {
    type Item = BigUint;
    fn next(&mut self) -> Option<Self::Item> {
        let next: BigUint = self.cur.clone() + &self.prev;
        self.prev = mem::replace(&mut self.cur, next);
        Some(self.prev.clone())
    }
}

pub fn solve_fast(num_digits: usize) -> usize {
    // Observe that F_n \approx \phi^n / \sqrt{5} for large n
    // We want to find n such that F_n >= 10^(num_digits - 1)
    // --> phi^n >= 10^(num_digits - 1) * sqrt{5}
    // --> n * Log[Phi] >= (num_digits - 1)*Log[10] + 1/2 * Log[5]
    // --> n >= (num_digits - 1) * Log[10] / Log[Phi] + 1/2 * Log[5] / Log[Phi]

    let phi = 0.5 * (1.0 + 5f64.sqrt());
    let n = (num_digits - 1) as f64 * 10.0_f64.ln() / phi.ln() + 0.5 * 5.0_f64.ln() / phi.ln();
    n as usize
}

#[test]
fn small() {
    assert_eq!(solve(3), 11);
}

#[test]
fn main() {
    assert_eq!(solve(1000), 4781);
}

#[test]
fn fast() {
    assert_eq!(solve_fast(1000), 4781);
}
