extern crate project_euler;
use project_euler::problems::problem_015;

#[test]
fn small() {
    assert_eq!(problem_015::solve(2, 2), 6);
}

#[test]
fn main() {
    assert_eq!(problem_015::solve(20, 20), 137846528820);
}
