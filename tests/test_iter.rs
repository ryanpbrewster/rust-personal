extern crate project_euler;

use project_euler::util::iter::Cross;
use project_euler::util::iter::imerge;

#[test]
fn crossiter_finite() {
    let crossed: Vec<_> = Cross::of(1..3, 4..6).collect();
    assert!(crossed == vec![(1, 4), (1, 5), (2, 4), (2, 5)]);
}

#[test]
fn imerge_works() {
    let xs = vec![1, 3, 5, 7, 9];
    let ys = vec![2, 3, 4, 8, 9, 10];
    let mut iter = imerge(xs.iter(), ys.iter());

    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&9));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}
