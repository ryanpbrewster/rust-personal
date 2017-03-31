extern crate num;

#[test]
fn small() {
    assert_eq!(solve(10), 2520);
}

#[test]
fn main() {
    assert_eq!(solve(20), 232792560);
}

pub fn solve(n: u32) -> u32 {
    (1..n).fold(1, num::integer::lcm)
}
