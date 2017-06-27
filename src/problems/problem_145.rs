use rayon::prelude::*;

// Count how many reversible numbers there are `< hi`
pub fn solve(hi: u64) -> u64 {
    (0..hi)
        .into_par_iter()
        .filter(|&n| is_reversible(n))
        .count() as u64
}

fn is_reversible(n: u64) -> bool {
    // Leading zeroes are not permitted.
    if n % 10 == 0 {
        return false;
    }

    let mut t = n;
    let mut r = 0;
    while t > 0 {
        r = 10 * r + t % 10;
        t /= 10;
    }

    let mut t = n + r;
    while t > 0 {
        if t % 2 == 0 {
            return false;
        }
        t /= 10;
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        assert_eq!(solve(1_000), 120);
    }

    #[test]
    fn main() {
        assert_eq!(solve(100_000_000), 608720);
    }
}