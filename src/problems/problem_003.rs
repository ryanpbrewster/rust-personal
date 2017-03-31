use util::primes;

#[test]
fn main() {
    assert!(solve(600851475143) == 6857);
}

pub fn solve(n: u64) -> u64 {
    primes::factors(n).into_iter().last().expect("no factors!")
}
