use util::math;

#[test]
fn small() {
    assert_eq!(solve(2, 2), 6);
}

#[test]
fn main() {
    assert_eq!(solve(20, 20), 137846528820);
}

pub fn solve(height: u64, width: u64) -> u64 {
    math::choose(height + width, width)
}
