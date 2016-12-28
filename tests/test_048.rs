extern crate project_euler;
use project_euler::problems::problem_048;

#[test]
fn small() {
    assert_eq!(problem_048::solve(10), 0405071317);
}

#[test]
fn main() {
    assert_eq!(problem_048::solve(1_000), 9110846700);
}
