extern crate project_euler;
use project_euler::problems::problem_050;

#[test]
fn small() {
    assert_eq!(problem_050::solve(100), 2 + 3 + 5 + 7 + 11 + 13);
}

#[test]
fn main() {
    assert_eq!(problem_050::solve(1_000_000), 997651);
}
