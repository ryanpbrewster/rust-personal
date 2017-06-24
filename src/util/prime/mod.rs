use util::iter::Group;

pub fn sieve(hi: usize) -> Vec<bool> {
    let mut is_prime_vec: Vec<bool> = vec![true; hi];
    is_prime_vec[0] = false;
    is_prime_vec[1] = false;
    for p in 2..hi {
        if is_prime_vec[p] {
            for k in p..(hi + p - 1) / p {
                is_prime_vec[k * p] = false;
            }
        }
    }
    is_prime_vec
}

pub fn test(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in (2..).take_while(|&i| i*i <= n) {
        if n % i == 0 {
            return false;
        }
    }
    true
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

pub fn num_divisors(n: u64) -> u32 {
    Group::of(factors(n)).map(|(_, k)| k + 1).fold(
        1,
        |a, b| a * b,
    ) as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn primes_iter() {
        assert_eq!(
            Primes::all().take(5).collect::<Vec<_>>(),
            vec![2, 3, 5, 7, 11]
        );
    }

    #[test]
    fn primes_iter_correctness() {
        let sieve: Vec<bool> = sieve(1_000);
        let ps: Vec<u32> = (2..1000).filter(|&i| sieve[i as usize]).collect();
        assert_eq!(
            Primes::all().take_while(|&n| n < 1_000).collect::<Vec<_>>(),
            ps
        );
    }

    #[test]
    fn primes_num_divisors() {
        // assert_eq!(primes::num_divisors(28), 6);
        assert_eq!(num_divisors(36), 9);
    }
}
