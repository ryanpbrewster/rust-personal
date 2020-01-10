use crate::util::prime;
use std::collections::{HashSet, VecDeque};

pub fn solve(consecutive: usize, num_factors: usize) -> u64 {
    // Find the first instance of `consecutive` consecutive integers, which each
    // have `num_factors` unique prime factors. Return the first element of that set.
    let mut xs: VecDeque<HashSet<u64>> = VecDeque::new();
    let mut n = 2;
    loop {
        while xs.len() < consecutive {
            xs.push_back(prime::factors(n).collect());
            n += 1;
        }
        assert!(xs.len() == consecutive);

        if xs.iter().all(|fs| fs.len() == num_factors) {
            return n - consecutive as u64;
        }
        xs.pop_front();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        assert_eq!(solve(2, 2), 14);
        assert_eq!(solve(3, 3), 644);
    }

    #[test]
    fn main() {
        assert_eq!(solve(4, 4), 134043);
    }
}
