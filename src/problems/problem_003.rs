use util::prime;

#[test]
fn main() {
    assert!(solve(600851475143) == 6857);
}

pub fn solve(n: u64) -> u64 {
    prime::factors(n).into_iter().last().expect("no factors!")
}
