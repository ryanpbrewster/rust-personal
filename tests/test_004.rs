extern crate project_euler;

use project_euler::problems::problem_004;

#[test]
fn small() {
	assert_eq!(problem_004::solve(1), 9);
}

#[test]
fn main() {
	assert_eq!(problem_004::solve(3), 906609);
}
