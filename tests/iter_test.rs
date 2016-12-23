extern crate project_euler;

use project_euler::util::iter;

#[test]
fn crossiter_finite() {
    let my_cross = iter::cross(1..3, 4..6);
    let crossed: Vec<_> = my_cross.collect();
    assert!(crossed == vec![(1, 4), (1, 5), (2, 4), (2, 5)]);
}
