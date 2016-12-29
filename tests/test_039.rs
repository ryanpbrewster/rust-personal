extern crate project_euler;
use project_euler::problems::problem_039;

#[test]
fn main() {
    assert_eq!(problem_039::solve(1000), 840);
}
