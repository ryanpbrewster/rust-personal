pub struct Primes {
    sieve_arr: Vec<bool>,
    sieve_start: usize,
    idx: usize,
    past: Vec<usize>,
}

impl Primes {
    pub fn all() -> Primes {
        Primes {
            sieve_arr: vec![true],
            sieve_start: 2,
            idx: 2,
            past: Vec::new(),
        }
    }

    fn end(&self) -> usize {
        self.sieve_start + self.sieve_arr.len()
    }
    fn extend_sieve(&mut self) {
        // Clear and double the size of the sieve
        self.sieve_start += self.sieve_arr.len();
        for i in self.sieve_arr.iter_mut() {
            *i = true;
        }
        let new_len = 2 * self.sieve_arr.len();
        self.sieve_arr.resize(new_len, true);

        let end = self.end();
        for &p in self.past.iter().take_while(|&p| p * p < end) {
            let mut i = (self.sieve_start + p - 1) / p * p - self.sieve_start;
            while i < self.sieve_arr.len() {
                self.sieve_arr[i] = false;
                i += p;
            }
        }
    }
    fn increment(&mut self) {
        self.idx += 1;
        if self.idx >= self.end() {
            self.extend_sieve();
        }
    }
}

impl Iterator for Primes {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        while !self.sieve_arr[self.idx - self.sieve_start] {
            self.increment();
        }
        let p = self.idx;
        self.past.push(p);
        self.increment();
        Some(p as u32)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        let ps: Vec<u32> = Primes::all().take(20).collect();
        assert_eq!(
            ps,
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,]
        );
    }
}
