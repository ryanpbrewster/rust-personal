use util::prime;

// Starting with 1 and spiralling anticlockwise in the following way, a square
// spiral with side length 7 is formed.
//
//   37 36 35 34 33 32 31
//   38 17 16 15 14 13 30
//   39 18  5  4  3 12 29
//   40 19  6  1  2 11 28
//   41 20  7  8  9 10 27
//   42 21 22 23 24 25 26
//   43 44 45 46 47 48 49
//
// It is interesting to note that the odd squares lie along the bottom right
// diagonal, but what is more interesting is that 8 out of the 13 numbers lying
// along both diagonals are prime; that is, a ratio of 8/13 ≈ 62%.
//
// If one complete new layer is wrapped around the spiral above, a square
// spiral with side length 9 will be formed. If this process is continued, what
// is the side length of the square spiral for which the ratio of primes along
// both diagonals first falls below 10%?

pub fn solve(bound: f64) -> u64 {
    let level = (1..)
        .map(|lvl| {
            // At each new level of the spiral we have 4 diagonal entries.
            // The last one (lower right) is 9, 25, 49, ... == (2l+1)^2
            // The other entries are lower by 2l (e.g., l=3 => 49 - 6 == 43).
            let end = (2 * lvl + 1) * (2 * lvl + 1);
            let diagonals = (0..4).map(|i| end - 2 * i * lvl);
            diagonals.filter(|&n| prime::test(n)).count()
        })
        .scan(0, |acc, v| {
            // Create a cumulative sum.
            let t = *acc;
            *acc += v;
            Some(t)
        })
        .enumerate()
        .skip(1)
        .find(|&(lvl, cnt)| {
            (cnt as f64) / (1.0 + 4.0 * lvl as f64) < bound
        })
        .unwrap()
        .0 as u64;

    2 * level + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main() {
        assert_eq!(solve(0.10), 26241);
    }
}
