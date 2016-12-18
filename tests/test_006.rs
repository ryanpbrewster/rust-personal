extern crate project_euler;

use project_euler::problems::problem_006;

#[test]
fn small() {
	assert_eq!(problem_006::solve(10), 2640);
}

#[test]
fn main() {
	assert_eq!(problem_006::solve(100), 25164150);
}
