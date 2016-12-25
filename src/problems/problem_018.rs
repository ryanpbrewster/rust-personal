use util::triangle::Triangle;
use std::cmp::max;

pub fn solve(t0: Triangle<i32>) -> i32 {
    let mut t = t0;
    for i in (0..t.height() - 1).rev() {
        for j in 0..i + 1 {
            t[(i, j)] += max(t[(i + 1, j)], t[(i + 1, j + 1)]);
        }
    }
    t[(0, 0)]
}
