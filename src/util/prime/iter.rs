pub struct Primes {
    sieve_arr: Vec<bool>,
    idx: usize,
}

impl Primes {
    pub fn all() -> Primes {
        Primes {
            sieve_arr: vec![false, false, true],
            idx: 1,
        }
    }
    fn extend_sieve(&mut self) {
        let new_len = 2 * self.sieve_arr.len();
        self.sieve_arr.resize(new_len, true);

        for i in (2..).take_while(|&i| i * i < new_len) {
            if self.sieve_arr[i] {
                for j in (i..).take_while(|&j| i * j < new_len) {
                    self.sieve_arr[i * j] = false;
                }
            }
        }
    }
}

impl Iterator for Primes {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        loop {
            if self.idx >= self.sieve_arr.len() {
                self.extend_sieve();
            }
            if self.sieve_arr[self.idx] {
                break;
            }
            self.idx += 1;
        }
        Some(self.idx as u32)
    }
}
