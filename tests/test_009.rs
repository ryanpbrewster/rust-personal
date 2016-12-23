extern crate project_euler;
use project_euler::problems::problem_009;

#[test]
fn small() {
	assert_eq!(problem_009::solve(3+4+5), Some(3*4*5));
}

#[test]
fn main() {
	assert_eq!(problem_009::solve(1_000), Some(31875000));
}
