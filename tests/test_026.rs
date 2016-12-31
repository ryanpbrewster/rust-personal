extern crate project_euler;
use project_euler::problems::problem_026;

#[test]
fn small() {
    assert_eq!(problem_026::solve(10), 7);
}

#[test]
fn main() {
    assert_eq!(problem_026::solve(1000), 983);
}
