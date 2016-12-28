use util::primes;

pub fn solve(hi: usize) -> usize {
    // Find the prime, p, where:
    //   - p < hi and
    //   - p == Sum[Prime[i], {i, a, b}]
    // that maximizes |b-a|
    let sieve = primes::sieve(hi);

    let ps: Vec<usize> = (2..hi).filter(|&i| sieve[i]).collect();
    let cum_sum: Vec<usize> = ps.iter()
        .scan(0, |acc, v| {
            *acc = *acc + v;
            Some(*acc - v)
        })
        .collect();

    let n_max = cum_sum.iter()
        .enumerate()
        .find(|&(_, v)| *v >= hi)
        .map(|(idx, _)| idx)
        .unwrap_or(ps.len());

    for n in (1..n_max).rev() {
        let end = (0..ps.len() - n)
            .find(|&i| cum_sum[i + n] - cum_sum[i] >= hi)
            .unwrap_or(ps.len() - n);
        for start in 0..end {
            let total = cum_sum[start + n] - cum_sum[start];
            if total < hi && sieve[total] {
                return total;
            }
        }
    }
    return 0;
}
