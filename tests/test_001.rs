extern crate project_euler;

use project_euler::problems::problem_001;

#[test]
fn test_001() {
	assert!(problem_001::solve(1000) == 233168);
}
