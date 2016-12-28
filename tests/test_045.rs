extern crate project_euler;
use project_euler::problems::problem_045;

#[test]
fn small() {
    assert_eq!(problem_045::solve(0), 1);
    assert_eq!(problem_045::solve(1), 40755);
}

#[test]
fn main() {
    assert_eq!(problem_045::solve(40755), 1533776805);
}
