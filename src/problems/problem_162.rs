/*
In the hexadecimal number system numbers are represented using 16 different
digits:

0,1,2,3,4,5,6,7,8,9,A,B,C,D,E,F

The hexadecimal number AF when written in the decimal number system equals
10x16+15=175.

In the 3-digit hexadecimal numbers 10A, 1A0, A10, and A01 the digits 0,1 and A
are all present. Like numbers written in base ten we write hexadecimal numbers
without leading zeroes.

How many hexadecimal numbers containing at most sixteen hexadecimal digits
exist with all of the digits 0,1, and A present at least once?  Give your
answer as a hexadecimal number.

(A,B,C,D,E and F in upper case, without any leading or trailing code that marks
the number as hexadecimal and without leading zeroes , e.g. 1A3F and not: 1a3f
and not 0x1a3f and not $1A3F and not #1A3F and not 0000001A3F)
*/

use num::{BigUint, One, Zero};

pub fn solve(max_digits: usize) -> BigUint {
    let mut tallies: Vec<BigUint> = (0..1 << 3).map(|_| BigUint::zero()).collect();
    tallies[0b000] = BigUint::from(13u32);
    tallies[0b001] = BigUint::one();
    tallies[0b010] = BigUint::one();

    let mut count = BigUint::zero();
    for _ in 1..max_digits {
        let mut tmp: Vec<BigUint> = (0..1 << 3).map(|_| BigUint::zero()).collect();
        tmp[0b000] = BigUint::from(13u32) * &tallies[0b000];
        tmp[0b001] = BigUint::from(14u32) * &tallies[0b001] + &tallies[0b000];
        tmp[0b010] = BigUint::from(14u32) * &tallies[0b010] + &tallies[0b000];
        tmp[0b011] = BigUint::from(15u32) * &tallies[0b011] + &tallies[0b010] + &tallies[0b001];
        tmp[0b100] = BigUint::from(14u32) * &tallies[0b100] + &tallies[0b000];
        tmp[0b101] = BigUint::from(15u32) * &tallies[0b101] + &tallies[0b100] + &tallies[0b001];
        tmp[0b110] = BigUint::from(15u32) * &tallies[0b110] + &tallies[0b100] + &tallies[0b010];
        tmp[0b111] = BigUint::from(16u32) * &tallies[0b111] + &tallies[0b110] + &tallies[0b101] +
            &tallies[0b011];
        tallies = tmp;
        count = count + &tallies[0b111];
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        assert_eq!(solve(0), BigUint::zero());
        assert_eq!(solve(1), BigUint::zero());
        assert_eq!(solve(2), BigUint::zero());
        assert_eq!(solve(3), BigUint::from(4u32)); // 0x10A, 0x1A0, 0xA01, 0xA10
    }

    #[test]
    fn main() {
        assert_eq!(
            solve(16).to_str_radix(16).to_uppercase(),
            "3D58725572C62302"
        );
    }
}
