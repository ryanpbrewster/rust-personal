use util::primes::Primes;

#[test]
fn small() {
    assert_eq!(solve(6), 13);
}

#[test]
fn main() {
    assert_eq!(solve(10_001), 104743);
}

#[test]
fn huge() {
    // Currently too slow to test
    // assert_eq!(problem_007::solve(1_000_001), 104743);
}

pub fn solve(n: u32) -> u32 {
    Primes::all().nth((n - 1) as usize).unwrap()
}
