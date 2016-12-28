extern crate project_euler;
use project_euler::problems::problem_025;

#[test]
fn small() {
    assert_eq!(problem_025::solve(3), 11);
}

#[test]
fn main() {
    assert_eq!(problem_025::solve(1000), 4781);
}

#[test]
fn fast() {
    assert_eq!(problem_025::solve_fast(1000), 4781);
}
