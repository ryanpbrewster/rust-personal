extern crate project_euler;
use project_euler::problems::problem_014;

#[test]
fn small() {
    assert_eq!(problem_014::solve(1..20), 19);
}

#[test]
fn main() {
    assert_eq!(problem_014::solve(1..1_000_000), 837799);
}

#[test]
fn fast() {
    assert_eq!(problem_014::solve_fast(1..1_000_000), 837799);
}
