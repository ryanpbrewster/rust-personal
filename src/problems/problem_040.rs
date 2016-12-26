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

pub fn solve_iter() -> u32 {
    let idxs = vec![0, 1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];
    let diffs: Vec<_> = idxs.windows(2).map(|w| w[1] - w[0]).collect();

    let mut champ = Champernowne::all();
    diffs.into_iter().map(|diff| champ.nth(diff-1).unwrap()).product()
}
