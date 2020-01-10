use crate::util::math;

#[test]
fn small() {
    assert_eq!(solve(10), 2640);
}

#[test]
fn main() {
    assert_eq!(solve(100), 25164150);
}

pub fn solve(n: u32) -> u32 {
    let s = math::sum(n);
    let s_sq = math::sum_squares(n);

    s * s - s_sq
}
