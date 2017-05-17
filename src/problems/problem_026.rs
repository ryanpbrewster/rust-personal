use util::math::Decimals;

pub fn solve(hi: u32) -> u32 {
    (2..hi).max_by_key(|&i| Decimals::reciprocal(i).repeating().1.len()).unwrap()
}

#[test]
fn small() {
    assert_eq!(solve(10), 7);
}

#[test]
fn main() {
    assert_eq!(solve(1000), 983);
}
