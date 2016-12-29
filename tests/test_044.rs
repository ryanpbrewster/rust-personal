extern crate project_euler;
use project_euler::problems::problem_044;

#[test]
fn small() {
    assert_eq!(problem_044::solve(), 5482660);
}
