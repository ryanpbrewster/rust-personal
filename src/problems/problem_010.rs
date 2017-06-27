use util::prime;

#[test]
fn small() {
    assert_eq!(solve(10), 17);
}

#[test]
fn main() {
    assert_eq!(solve(2_000_000), 142913828922);
}

// Sum of all primes < hi
pub fn solve(hi: u32) -> u64 {
    prime::all().take_while(|&p| p < hi).fold(
        0,
        |a, b| a + b as u64,
    )
}
