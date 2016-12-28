extern crate project_euler;
use project_euler::problems::problem_030;

#[test]
fn main() {
    assert_eq!(problem_030::solve(), 443839);
}
