extern crate project_euler;
use project_euler::problems::problem_010;

#[test]
fn small() {
	assert_eq!(problem_010::solve(10), 17);
}

#[test]
fn main() {
	assert_eq!(problem_010::solve(2_000_000), 142913828922);
}
