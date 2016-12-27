extern crate project_euler;
use project_euler::problems::problem_020;

#[test]
fn small() {
    assert_eq!(problem_020::solve(10), 27);
}

#[test]
fn main() {
    assert_eq!(problem_020::solve(100), 648);
}
