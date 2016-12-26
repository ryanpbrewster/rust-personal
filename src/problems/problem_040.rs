use util::math::Digits;

pub struct Champernowne {
    n: u32,
    digits: Vec<u32>,
}

impl Champernowne {
    pub fn all() -> Champernowne {
        Champernowne {
            n: 0,
            digits: Vec::new(),
        }
    }
}

impl Iterator for Champernowne {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.digits.pop() {
            Some(d) => Some(d),
            None => {
                self.n += 1;
                self.digits = Digits::decimal(self.n).collect();
                self.digits.pop()
            }
        }
    }
}

pub fn solve() -> u32 {
    let idxs = vec![1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];
    let digits: Vec<_> = Champernowne::all().take(1_000_000).collect();
    idxs.into_iter().map(|idx| digits[idx-1]).product()
}
