use util::math::pythag::Triple;
use std::collections::VecDeque;

// Find any pythagorean triple where a+b+c == n
pub fn solve(n: u32) -> Option<u32> {
    let mut q = VecDeque::new();
    q.push_back(Triple::root());

    while let Some(t) = q.pop_front() {
        // We only explore primitive triples, so we need to check
        // not just if a+b+c == n, but if any scaled version of this triple
        // may work (l*a, l*b, l*c) --> l*(a+b+c) == n --> n%(a+b+c) == 0
        if n % t.sum() == 0 {
            return Some(t.scaled(n / t.sum()).product());
        }
        for child in t.branch().filter(|ch| ch.sum() <= n) {
            q.push_back(child);
        }
    }
    None
}
