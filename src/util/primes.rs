pub fn sieve(hi: u32) -> Vec<u32> {
    let mut is_prime_vec : Vec<bool> = vec![true; hi as usize];
    for p in 2..hi {
        if is_prime_vec[p as usize] {
            for k in p..(hi+p-1)/p {
                is_prime_vec[(k*p) as usize] = false;
            }
        }
    }
    (2..hi).filter(|&p| is_prime_vec[p as usize]).collect()
}

pub struct Primes {
    sieve_arr: Vec<bool>,
    idx: usize,
}

impl Primes {
    fn extend_sieve(&mut self) {
        let new_len = 2*self.sieve_arr.len();
        self.sieve_arr.resize(new_len, true);

        for i in (2..) .take_while(|&i| i*i < new_len) {
            if self.sieve_arr[i] {
                for j in (i..).take_while(|&j| i*j < new_len) {
                    self.sieve_arr[i*j] = false;
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

pub fn primes() -> Primes {
    Primes {
        sieve_arr: vec![false, false, true],
        idx: 1,
    }
}

#[allow(dead_code)]
fn trial_division(n: u32) -> bool {
    if n == 2 {
        return true;
    }
    if n%2 == 0 {
        return false;
    }

    let mut i = 3;
    while i*i <= n {
        if n%i == 0 {
            return false;
        }
        i += 1;
    }
    true
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
