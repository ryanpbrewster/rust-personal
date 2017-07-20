// Find the smallest n such that P(n) === 0 (mod 10^6) where P(n) is the integer partition function

pub fn solve(m: i32) -> usize {
    let mut partitions = vec![1];
    for i in 1.. {
        let mut p = 0;
        for (idx, j) in Pentagonals::new().take_while(|&j| j <= i).enumerate() {
            if idx % 4 < 2 {
                p += partitions[(i - j) as usize];
            } else {
                p -= partitions[(i - j) as usize];
            }
            p %= m;
        }
        partitions.push(p);
        if p == 0 {
            return i as usize;
        }
    }
    panic!("Unreachable")
}

pub struct Pentagonals {
    sign: i32,
    m: i32
}

impl Pentagonals {
    fn new() -> Pentagonals {
        Pentagonals {
            sign: -1,
            m: 1,
        }
    }
}

impl Iterator for Pentagonals {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let t = self.m * (3 * self.m + self.sign) / 2;
        if self.sign < 0 {
            self.sign = 1;
        } else {
            self.sign = -1;
            self.m += 1;
        }
        Some(t)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pentagonals_manual() {
        assert_eq!(
            Pentagonals::new().take(20).collect::<Vec<_>>(),
            vec![1, 2, 5, 7, 12, 15, 22, 26, 35, 40, 51, 57, 70, 77, 92, 100, 117, 126, 145, 155]
        );
    }

    #[test]
    fn main() {
        assert_eq!(solve(1_000_000), 55374);
    }
}
