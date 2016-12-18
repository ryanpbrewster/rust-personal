extern crate project_euler;

use project_euler::problems::problem_005;

#[test]
fn small() {
	assert_eq!(problem_005::solve(10), 2520);
}

#[test]
fn main() {
	assert_eq!(problem_005::solve(20), 232792560);
}
