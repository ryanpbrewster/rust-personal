use util::prime;

pub fn solve(hi: usize) -> usize {
    // Find the prime, p, where:
    //   - p < hi and
    //   - p == Sum[Prime[i], {i, a, b}]
    // that maximizes |b-a|
    let sieve = prime::sieve(hi);

    let ps: Vec<usize> = (2..hi).filter(|&i| sieve[i]).collect();
    let cum_sum: Vec<usize> = ps.iter()
        .scan(0, |acc, v| {
            *acc = *acc + v;
            Some(*acc - v)
        })
        .collect();

    // What is the maximum number of consecutive primes we should consider?
    let n_max = cum_sum
        .iter()
        .enumerate()
        .find(|&(_, v)| *v >= hi)
        .map(|(idx, _)| idx)
        .unwrap_or_else(|| ps.len());

    // For each possible number of consecutive primes (in descending order),
    // check if there are any prime sums. Return the first one.
    (1..n_max)
        .rev()
        .flat_map(|n| {
            (0..ps.len() - n)
                .map(|i| cum_sum[i + n] - cum_sum[i])
                .take_while(|&t| t < hi)
                .find(|&t| sieve[t])
        })
        .next()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        assert_eq!(solve(100), 2 + 3 + 5 + 7 + 11 + 13);
    }

    #[test]
    fn main() {
        assert_eq!(solve(1_000_000), 997651);
    }
}
