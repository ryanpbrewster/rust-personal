extern crate project_euler;

use project_euler::util::math::pythag;

#[test]
fn pythag_triple() {
    assert!(pythag::Triple::check(1, 2, 3).is_none());
    assert!(pythag::Triple::check(3, 4, 5).is_some());
}

#[test]
fn pythag_triple_tree_branch() {
    assert_eq!(
        pythag::Triple::new(3, 4, 5).branch().collect::<Vec<_>>(),
        vec![
            pythag::Triple::new(8, 15, 17),
            pythag::Triple::new(20, 21, 29),
            pythag::Triple::new(5, 12, 13),
        ]
    );
}

#[test]
fn pythag_triple_scaling() {
    assert_eq!(
        pythag::Triple::new(3, 4, 5).scaled(11),
        pythag::Triple::new(33, 44, 55)
    );
    assert_eq!(
        pythag::Triple::new(3, 4, 5)
            .scaled_triples()
            .nth(10)
            .unwrap(),
        pythag::Triple::new(33, 44, 55)
    );
}
