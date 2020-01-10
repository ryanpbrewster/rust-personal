pub fn solve() -> usize {
    for i in 2.. {
        for j in 1..i {
            let pi = f(i);
            let pj = f(j);
            if f_inverse(pi + pj).is_some() && f_inverse(pi + 2 * pj).is_some() {
                return pi;
            }
        }
    }
    0
}

fn f(n: usize) -> usize {
    n * (3 * n - 1) / 2
}

fn f_inverse(p: usize) -> Option<usize> {
    let n: usize = (1.0 / 6.0 * (1.0 + (24.0 * p as f64 + 1.0).sqrt())) as usize;
    if f(n) == p {
        Some(n)
    } else {
        None
    }
}

#[test]
fn small() {
    assert_eq!(solve(), 5482660);
}
