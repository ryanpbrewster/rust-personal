extern crate project_euler;
use project_euler::problems::problem_024;

#[test]
fn small() {
    assert_eq!(problem_024::solve(vec![1, 2, 3], 0), vec![1, 2, 3]);
    assert_eq!(problem_024::solve(vec![1, 2, 3], 2), vec![2, 1, 3]);
    assert_eq!(problem_024::solve(vec![1, 2, 3], 6), vec![1, 2, 3]);
}

#[test]
fn main() {
    assert_eq!(problem_024::solve((0..10).collect(), 1_000_000 - 1),
               vec![2, 7, 8, 3, 9, 1, 5, 4, 6, 0]);
}
