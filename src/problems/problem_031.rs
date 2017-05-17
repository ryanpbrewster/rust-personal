use util::coins;
use std::collections::HashSet;

pub fn solve(n: usize) -> u32 {
    let cs: HashSet<usize> = vec![1, 2, 5, 10, 20, 50, 100, 200].into_iter().collect();
    coins::ways_to_make_with_replacement(n, &cs)
}

#[test]
fn main() {
    assert_eq!(solve(200), 73682);
}
