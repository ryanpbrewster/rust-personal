use crate::util::math::pythag::Triple;
use std::collections::VecDeque;

#[test]
fn small() {
    assert_eq!(solve(3 + 4 + 5), Some(3 * 4 * 5));
}

#[test]
fn main() {
    assert_eq!(solve(1_000), Some(31875000));
}

#[test]
fn no_valid_answer() {
    assert_eq!(solve(999), None);
}

// Find any pythagorean triple where a+b+c == n
pub fn solve(n: u32) -> Option<u32> {
    let mut q: VecDeque<Triple> = VecDeque::new();
    q.push_back(Triple::root());

    while let Some(t) = q.pop_front() {
        let p = t.sum();
        if p <= n {
            // We only explore primitive triples, so we need to check
            // not just if a+b+c == n, but if any scaled version of this triple
            // may work (l*a, l*b, l*c) --> l*(a+b+c) == n --> n%(a+b+c) == 0
            if n % p == 0 {
                let l = n / p;
                return Some(l * l * l * t.product());
            }
            for child in t.branch() {
                q.push_back(child);
            }
        }
    }
    None
}
