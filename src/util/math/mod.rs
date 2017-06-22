use std::cmp::min;
use std::convert::From;
use num::Integer;

pub mod pythag;

// Sum[i, {i, 1, n}]
pub fn sum(n: u32) -> u32 {
    n * (n + 1) / 2
}

// Sum[i^2, {i, 1, n}]
pub fn sum_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn factorial(n: u64) -> u64 {
    (2..n + 1).fold(1, |a, b| a * b)
}

pub fn choose(n: u64, k: u64) -> u64 {
    // Observe that choose(n, k) is an integer for all n, k
    // Also observe that:
    // choose(n, k) == choose(n-1, k-1) * (n / k)
    //              == choose(n-2, k-2) * (n-1 / k-1) * (n / k)
    //              == choose(n-k, 0) * ... * (n-1 / k-1) * (n / k)
    //              == 1 * (n-k+1 / 1) * (n-k+2 / 2) * (n-k+3 / 3) * ... * (n-1 / k-1) * (n / k)

    // As a minor optimization, since choose(n, k) == choose(n, n-k),
    // we can switch between them to minimize the number of multiplies.
    let kk = min(k, n - k);
    (1..kk + 1).fold(1, |acc, i| acc * (n - kk + i) / i)
}

pub fn powmod<T>(b: &T, e: &T, m: &T) -> T
where
    T: Integer + Clone,
{
    if e.is_zero() {
        T::one()
    } else {
        let two = T::one() + T::one();
        let (e_div_2, e_mod_2) = e.div_mod_floor(&two);
        let t: T = powmod(b, &e_div_2, m);
        let t_sq: T = (t.clone() * t).mod_floor(m);
        if e_mod_2.is_zero() {
            t_sq
        } else {
            (t_sq * b.clone()).mod_floor(m)
        }
    }
}

pub struct Digits {
    n: u32,
    radix: u32,
}

impl Digits {
    pub fn of(n: u32, radix: u32) -> Digits {
        Digits { n: n, radix: radix }
    }

    pub fn decimal(n: u32) -> Digits {
        Digits::of(n, 10)
    }
}

impl Iterator for Digits {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            None
        } else {
            let d = self.n % self.radix;
            self.n /= self.radix;
            Some(d)
        }
    }
}


pub struct Decimals {
    n: u32,
    d: u32,
}

impl Decimals {
    pub fn reciprocal(d: u32) -> Decimals {
        Decimals { n: 1, d: d }
    }

    pub fn repeating(mut self) -> (Vec<u32>, Vec<u32>) {
        // the index that self.n was first seen
        let mut vis: Vec<Option<usize>> = vec![None; self.d as usize];
        vis[self.n as usize] = Some(0);

        let mut part: Vec<u32> = Vec::new();
        while let Some(digit) = self.next() {
            part.push(digit);
            match vis[self.n as usize] {
                None => vis[self.n as usize] = Some(part.len()),

                Some(idx) => return (Vec::from(&part[..idx]), Vec::from(&part[idx..])),
            };
        }
        (part, Vec::new())
    }
}

impl Iterator for Decimals {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let q = 10 * self.n / self.d;
        self.n = (10 * self.n) % self.d;
        Some(q)

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(10), 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9 * 10);
    }

    #[test]
    fn test_choose() {
        assert_eq!(choose(10, 0), 1);
        assert_eq!(choose(10, 1), 10);
        assert_eq!(choose(10, 2), 10 * 9 / 2);
        assert_eq!(choose(10, 5), (10 * 9 * 8 * 7 * 6) / (5 * 4 * 3 * 2));

        assert_eq!(choose(100_000, 99_999), 100_000);
        assert_eq!(choose(100_000, 100_000), 1);
    }
}
