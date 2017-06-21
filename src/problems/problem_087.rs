/*
The smallest number expressible as the sum of a prime square, prime cube, and
prime fourth power is 28. In fact, there are exactly four numbers below fifty
that can be expressed in such a way:

    28 = 2^2 + 2^3 + 2^4
    33 = 3^2 + 2^3 + 2^4
    49 = 5^2 + 2^3 + 2^4
    47 = 2^2 + 3^3 + 2^4

How many numbers below fifty million can be expressed as the sum of a prime
square, prime cube, and prime fourth power?
*/

use util::prime;

// Count how many numbers `n < hi` can be expressed as the sum of a prime
// square + prime cube + prime 4th?
fn solve(hi: usize) -> usize {
    let ps: Vec<usize> = prime::sieve(hi)
        .into_iter()
        .enumerate()
        .filter(|&(_, b)| b)
        .map(|(i, _)| i)
        .collect();
    let mut can_make: Vec<bool> = vec![false; hi];

    for p4 in ps.iter().map(|p| p * p * p * p).take_while(|&p4| p4 < hi) {
        for p3 in ps.iter().map(|p| p * p * p).take_while(|&p3| p4 + p3 < hi) {
            for p2 in ps.iter().map(|p| p * p).take_while(|&p2| p4 + p3 + p2 < hi) {
                can_make[p4 + p3 + p2] = true;
            }
        }
    }

    can_make.into_iter().filter(|&b| b).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        assert_eq!(solve(50), 4);
    }

    #[test]
    fn main() {
        assert_eq!(solve(50_000_000), 1097343);
    }
}
