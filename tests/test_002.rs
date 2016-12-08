extern crate project_euler;

use project_euler::problems::problem_002;

#[test]
fn test_002() {
	assert!(problem_002::solve(4_000_000) == 4613732);
}
