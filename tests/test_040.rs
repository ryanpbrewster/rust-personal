extern crate project_euler;
use project_euler::problems::problem_040;

#[test]
fn small() {
    assert_eq!(problem_040::Champernowne::all().take(20).collect::<Vec<_>>(),
               vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 1, 1, 1, 2, 1, 3, 1, 4, 1]);
}

#[test]
fn main() {
    assert_eq!(problem_040::solve(), 210);
}

#[test]
fn iter() {
    assert_eq!(problem_040::solve_iter(), 210);
}
