pub fn sieve(hi: u32) -> Vec<u32> {
    let mut is_prime_vec : Vec<bool> = vec![true; hi as usize];
    for p in 2..hi {
        if is_prime_vec[p as usize] {
            for k in p..hi/p {
                is_prime_vec[(k*p) as usize] = false;
            }
        }
    }
    (2..hi).filter(|&p| is_prime_vec[p as usize]).collect()
}

pub struct Factors {
    n: u64,
    lo: u64
}

pub fn factors(n: u64) -> Factors {
    Factors { n: n, lo: 2 }
}

impl Iterator for Factors {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n < self.lo {
            return None;
        }
        // Increment self.lo until it divides n
        while self.n % self.lo != 0 {
            if self.lo == 2 {
                self.lo = 3;
            } else if self.lo * self.lo > self.n {
                self.lo = self.n;
            } else {
                self.lo += 2;
            }
        }
        self.n /= self.lo;
        Some(self.lo)
    }
}
