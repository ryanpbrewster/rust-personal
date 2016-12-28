use util::math::Digits;

pub fn solve() -> u32 {
    // Observe that there is no n >= 10^6 such that f(n) == n.
    // Consider f(9_999_999) == 7 * 9^5 == 413_343, so no 7-digit number
    // will ever map to a 7-digit number. Thus, we only need to consider
    // n < 10^6.
    // We can reduce this slightly further, with diminishing returns.
    (10..413_343 + 1).filter(|&n| f(n) == n).sum()
}

fn f(n: u32) -> u32 {
    Digits::decimal(n).map(|d| d * d * d * d * d).sum()
}
