extern crate project_euler;

use project_euler::problems::problem_003;

#[test]
fn test_003() {
	assert!(problem_003::solve(600851475143) == 6857);
}
