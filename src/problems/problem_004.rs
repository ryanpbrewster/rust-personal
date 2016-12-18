use std::cmp::max;

const LOWER_BOUNDS: [u32; 6] = [0, 1, 10, 100, 1000, 10000];

pub fn solve(n_digits: u32) -> u32 {
    assert!(
        1 <= n_digits && n_digits <= 4,
        "n_digits ({}) invalid", n_digits);
    let lo = LOWER_BOUNDS[n_digits as usize];
    let hi = 10*lo;

    let mut best = 0;
    for a in lo..hi {
        // Only search for pairs (a,b)
        // where a*b > best
        // --> b >= best/a
        // and b <= a (to avoid double-searching pairs)
        for b in max(a, best/a)..hi {
            let n = a*b;
            if is_palindrome(n.to_string()) {
                best = n;
            }
        }
    }
    best
}

fn is_palindrome(str: String) -> bool {
    str == str.chars().rev().collect::<String>()
}
