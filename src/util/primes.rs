pub fn sieve(hi: u32) -> Vec<u32> {
    let mut is_prime_vec: Vec<bool> = vec![true; hi as usize];
    for p in 2..hi {
        if is_prime_vec[p as usize] {
            for k in p..(hi + p - 1) / p {
                is_prime_vec[(k * p) as usize] = false;
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

pub struct Factors {
    n: u64,
    lo: u64,
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

struct Group<S: Iterator> {
    source: S,
    prev: Option<S::Item>,
}

impl<S: Iterator> Group<S> {
    fn of(mut source: S) -> Group<S> {
        let prev = source.next();
        Group {
            source: source,
            prev: prev,
        }
    }
}

impl<S> Iterator for Group<S>
    where S: Iterator,
          S::Item: Eq
{
    type Item = (S::Item, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.prev.take().map(|x| {
            let mut count = 1;
            while let Some(cur) = self.source.next() {
                if cur == x {
                    count += 1;
                } else {
                    self.prev = Some(cur);
                    break;
                }
            }
            (x, count)
        })
    }
}

pub fn num_divisors(n: u64) -> u32 {
    Group::of(factors(n)).map(|(_, k)| k + 1).fold(1, |a, b| a * b) as u32
}
