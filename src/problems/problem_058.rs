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
    let mut yes = 0;
    let mut no = 1;

    println!("solving for {}", bound);

    let mut spiral = Spiral::new();
    while let Some(xs) = spiral.next() {
        for x in xs {
            if prime::test(x) {
                yes += 1;
            } else {
                no += 1;
            }
        }
        let chi = (yes as f64) / (yes as f64 + no as f64);
        if chi < bound {
            return 2 * spiral.level - 1;
        }
    }

    0
}

struct Spiral {
    pub level: u64,
    n: u64
}

impl Spiral {
    pub fn new() -> Spiral {
        Spiral { level: 1, n: 3 }
    }
}

impl Iterator for Spiral {
    type Item = Vec<u64>;
    fn next(&mut self) -> Option<Vec<u64>> {
        let xs: Vec<u64> = (0..4).map(|i| self.n + 2 * self.level * i).collect();
        self.n += 8 * self.level + 2;
        self.level += 1;
        Some(xs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn spiral() {
        let mut s = Spiral::new();
        assert_eq!(s.next(), Some(vec![3, 5, 7, 9]));
        assert_eq!(s.next(), Some(vec![13, 17, 21, 25]));
        assert_eq!(s.next(), Some(vec![31, 37, 43, 49]));
    }

    #[test]
    fn main() {
        assert_eq!(solve(0.10), 26241);
    }
}
