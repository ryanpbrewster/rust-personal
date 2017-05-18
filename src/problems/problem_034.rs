use util::math::Digits;

pub fn solve() -> u32 {
    // Consider f(99_999_999) == 8 * 9! == 2_903_040, so no 8-digit number
    // will ever map to a 8-digit number. Thus, we only need to consider
    // n < 10^7.
    // We can reduce this slightly further, with diminishing returns.
    (10..2_903_040 + 1).filter(|&n| f(n) == n).sum()
}

const FACTORIAL: [u32; 10] = [1,
                              1,
                              2,
                              2 * 3,
                              2 * 3 * 4,
                              2 * 3 * 4 * 5,
                              2 * 3 * 4 * 5 * 6,
                              2 * 3 * 4 * 5 * 6 * 7,
                              2 * 3 * 4 * 5 * 6 * 7 * 8,
                              2 * 3 * 4 * 5 * 6 * 7 * 8 * 9];
fn f(n: u32) -> u32 {
    Digits::decimal(n).map(|d| FACTORIAL[d as usize]).sum()
}

#[test]
fn main() {
    assert_eq!(solve(), 40730);
}
