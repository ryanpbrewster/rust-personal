use crate::util::prime;

#[test]
fn small() {
    assert_eq!(solve(5), 28);
}

#[test]
fn main() {
    assert_eq!(solve(500), 76576500);
}

// Find the first triangle number with > `lower_bound` divisors
pub fn solve(lower_bound: u32) -> u32 {
    (1..)
        .map(|i| i * (i + 1) / 2)
        .find(|&n| prime::num_divisors(n as u64) > lower_bound)
        .expect("couldn't find any triangle number with > lower_bound divisors")
}
