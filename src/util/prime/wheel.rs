// Specifically a [2;3;5;7] wheel
pub struct Wheel {
    n: u64,
    idx: usize,
}

const SPOKES: [u64; 48] = [
    10,
    2,
    4,
    2,
    4,
    6,
    2,
    6,
    4,
    2,
    4,
    6,
    6,
    2,
    6,
    4,
    2,
    6,
    4,
    6,
    8,
    4,
    2,
    4,
    2,
    4,
    8,
    6,
    4,
    6,
    2,
    4,
    6,
    2,
    6,
    6,
    4,
    2,
    4,
    6,
    2,
    6,
    4,
    2,
    4,
    2,
    10,
    2,
];
impl Wheel {
    pub fn new() -> Wheel {
        Wheel { n: 1, idx: 0 }
    }
}

impl Iterator for Wheel {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.n += SPOKES[self.idx % SPOKES.len()];
        self.idx += 1;
        Some(self.n)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Values produced by a [2;3;5;7] wheel should not be divisible by 2, 3, 5, or 7
    fn check(n: u64) -> bool {
        n % 2 != 0 && n % 3 != 0 && n % 5 != 0 && n % 7 != 0
    }

    #[test]
    fn definition() {
        for n in Wheel::new().take(1_000) {
            assert!(check(n));
        }
    }

    #[test]
    fn compactness() {
        // Ensure that Wheel produces exactly the set of numbers not divisible for 2, 3, 5, or 7
        let expected: Vec<u64> = (2..1000).filter(|&n| check(n)).collect();
        let actual: Vec<u64> = Wheel::new().take_while(|&n| n < 1000).collect();
        assert_eq!(actual, expected);
    }
}
