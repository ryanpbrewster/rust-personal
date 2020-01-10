use num;
use num::{BigUint, One, Zero};
use std::ops::Mul;

pub struct ModFibSeq {
    a: u64,
    b: u64,
    m: u64,
}

impl ModFibSeq {
    pub fn new(m: u64) -> ModFibSeq {
        ModFibSeq { a: 0, b: 1, m: m }
    }
}

impl Iterator for ModFibSeq {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let t = self.a;
        self.a = (self.a + self.b) % self.m;
        self.b = t;
        Some(t)
    }
}

pub struct BigFibSeq {
    a: BigUint,
    b: BigUint,
}

impl BigFibSeq {
    pub fn new() -> BigFibSeq {
        BigFibSeq {
            a: BigUint::zero(),
            b: BigUint::one(),
        }
    }
    pub fn nth(n: usize) -> BigUint {
        // MatrixPow([1, 1; 1, 0], n) == [F_{n+1}, F_n; F_n, F_{n-1}];
        num::pow(M22::new([[1, 1], [1, 0]]), n).b
    }
}

impl Iterator for BigFibSeq {
    type Item = BigUint;
    fn next(&mut self) -> Option<BigUint> {
        let t = self.a.clone();
        self.a = &self.a + &self.b;
        self.b = t.clone();
        Some(t)
    }
}

#[derive(Debug, Clone)]
struct M22 {
    a: BigUint,
    b: BigUint,
    c: BigUint,
    d: BigUint,
}

impl M22 {
    fn new(elems: [[u32; 2]; 2]) -> M22 {
        M22 {
            a: BigUint::from(elems[0][0]),
            b: BigUint::from(elems[0][1]),
            c: BigUint::from(elems[1][0]),
            d: BigUint::from(elems[1][1]),
        }
    }
}

impl One for M22 {
    fn one() -> M22 {
        M22 {
            a: BigUint::one(),
            b: BigUint::zero(),
            c: BigUint::zero(),
            d: BigUint::one(),
        }
    }
}

impl Mul for M22 {
    type Output = M22;
    ///
    /// ( a1 b1 ) ( a2 b2 )  ==  ( a1*a2+b1*c2  a1*b2+b1*d2 )
    /// ( c1 d1 ) ( c2 d2 )      ( c1*a2+d1*c2  c1*b2+d1*d2 )
    fn mul(self, rhs: M22) -> M22 {
        M22 {
            a: &self.a * &rhs.a + &self.b * &rhs.c,
            b: &self.a * &rhs.b + &self.b * &rhs.d,
            c: &self.c * &rhs.a + &self.d * &rhs.c,
            d: &self.c * &rhs.b + &self.d * &rhs.d,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use num::ToPrimitive;
    use std::str::FromStr;

    #[test]
    fn bigfib_manual() {
        let small_fibs: Vec<u32> = BigFibSeq::new()
            .take(12)
            .map(|ref n| n.to_u32().unwrap())
            .collect();
        assert_eq!(small_fibs, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);

        assert_eq!(
            BigFibSeq::new().nth(100).unwrap(),
            BigUint::from_str("354224848179261915075").unwrap()
        )
    }

    #[test]
    fn bigfibmat_manual() {
        assert_eq!(
            BigFibSeq::nth(100),
            BigUint::from_str("354224848179261915075").unwrap()
        )
    }

    #[test]
    fn bigfibmat_matches_bigfibseq() {
        let mut fibs = BigFibSeq::new();
        for i in 0..1_000 {
            assert_eq!(BigFibSeq::nth(i), fibs.next().unwrap())
        }
    }
}
