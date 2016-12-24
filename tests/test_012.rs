extern crate project_euler;
use project_euler::problems::problem_012;

#[test]
fn small() {
    assert_eq!(problem_012::solve(5), 28);
}

#[test]
fn main() {
    assert_eq!(problem_012::solve(500), 76576500);
}
