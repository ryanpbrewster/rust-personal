use crate::util::iter::Group;

pub mod checks;
pub mod iter;
pub mod wheel;

pub fn all() -> iter::Primes {
    iter::Primes::all()
}

pub fn test(n: u64) -> bool {
    checks::trial_division(n)
}

pub fn factors(n: u64) -> Factors {
    Factors { n: n, lo: 2 }
}

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

pub struct Factors {
    n: u64,
    lo: u64,
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
    Group::of(factors(n))
        .map(|(_, k)| k + 1)
        .fold(1, |a, b| a * b) as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn primes_iter() {
        assert_eq!(all().take(5).collect::<Vec<_>>(), vec![2, 3, 5, 7, 11]);
    }

    #[test]
    fn primes_iter_correctness() {
        let sieve: Vec<bool> = sieve(1_000);
        let ps: Vec<u32> = (2..1000).filter(|&i| sieve[i as usize]).collect();
        assert_eq!(all().take_while(|&n| n < 1_000).collect::<Vec<_>>(), ps);
    }

    #[test]
    fn primes_num_divisors() {
        // assert_eq!(primes::num_divisors(28), 6);
        assert_eq!(num_divisors(36), 9);
    }
}
