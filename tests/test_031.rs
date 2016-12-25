extern crate project_euler;
use project_euler::problems::problem_031;

#[test]
fn main() {
    assert_eq!(problem_031::solve(200), 73682);
}
