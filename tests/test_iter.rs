extern crate project_euler;

use project_euler::util::iter::Cross;

#[test]
fn crossiter_finite() {
    let crossed: Vec<_>  = Cross::of(1..3, 4..6).collect();
    assert!(crossed == vec![(1, 4), (1, 5), (2, 4), (2, 5)]);
}
