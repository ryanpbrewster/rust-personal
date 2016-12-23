extern crate project_euler;

use project_euler::problems::problem_007;

#[test]
fn small() {
    assert_eq!(problem_007::solve(6), 13);
}

#[test]
fn main() {
    assert_eq!(problem_007::solve(10_001), 104743);
}

#[test]
fn huge() {
    // Currently too slow to test
    // assert_eq!(problem_007::solve(1_000_001), 104743);
}
