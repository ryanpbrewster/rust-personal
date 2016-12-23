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
        pythag::Triple::new(3, 4, 5).branch(), 
        ( pythag::Triple::new( 8, 15, 17)
        , pythag::Triple::new(20, 21, 29)
        , pythag::Triple::new( 5, 12, 13)
        )
    );
}
