/*
The palindromic number 595 is interesting because it can be written as the sum
of consecutive squares: 6^2 + 7^2 + 8^2 + 9^2 + 10^2 + 11^2 + 12^2.

There are exactly eleven palindromes below one-thousand that can be written as
consecutive square sums, and the sum of these palindromes is 4164. Note that 1
= 0^2 + 1^2 has not been included as this problem is concerned with the squares
of positive integers.

Find the sum of all the numbers less than 10^8 that are both palindromic and can
be written as the sum of consecutive squares.
*/

use crate::util::math::Digits;
use std::collections::HashSet;

pub fn solve(bound: u32) -> u64 {
    let squares: Vec<u32> = (1..).map(|n| n * n).take_while(|&n| n < bound).collect();
    let mut palindromic_cumsums: HashSet<u32> = HashSet::new();
    for start in 0..squares.len() - 1 {
        let cumsums = squares[start + 1..]
            .iter()
            .scan(squares[start], |acc, &n| {
                *acc += n;
                Some(*acc)
            })
            .take_while(|&n| n < bound);
        for cs in cumsums {
            if is_palindrome(cs) {
                palindromic_cumsums.insert(cs);
            }
        }
    }
    palindromic_cumsums.into_iter().map(|n| n as u64).sum()
}

fn is_palindrome(n: u32) -> bool {
    let digits: Vec<u32> = Digits::decimal(n).collect();
    digits.iter().eq(digits.iter().rev())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        assert_eq!(solve(1_000), 4164);
    }

    #[test]
    fn main() {
        assert_eq!(solve(100_000_000), 2906969179);
    }
}
