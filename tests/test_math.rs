extern crate project_euler;

use project_euler::util::math;

#[test]
fn test_factorial() {
    assert_eq!(math::factorial(0), 1);
    assert_eq!(math::factorial(1), 1);
    assert_eq!(math::factorial(2), 2);
    assert_eq!(math::factorial(3), 6);
    assert_eq!(math::factorial(10), 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10);
}

#[test]
fn test_choose() {
    assert_eq!(math::choose(10, 0), 1);
    assert_eq!(math::choose(10, 1), 10);
    assert_eq!(math::choose(10, 2), 10 * 9 / 2);
    assert_eq!(math::choose(10, 5), (10 * 9 * 8 * 7 * 6) / (5 * 4 * 3 * 2));

    assert_eq!(math::choose(100_000, 99_999), 100_000);
    assert_eq!(math::choose(100_000, 100_000), 1);
}
