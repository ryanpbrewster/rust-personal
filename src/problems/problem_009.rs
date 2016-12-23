use util::math::pythag::Triple;

// Find any pythagorean triple where a+b+c == n
pub fn solve(n: u32) -> Option<u32> {
    for a in 1..n / 3 {
        for b in a + 1..n / 2 {
            if Triple::check(a, b, n - a - b).is_some() {
                return Some(a * b * (n - a - b));
            }
        }
    }
    None
}
