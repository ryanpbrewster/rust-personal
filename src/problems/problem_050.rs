use util::primes;

pub fn solve(hi: usize) -> usize {
    // Find the prime, p, where:
    //   - p < hi and
    //   - p == Sum[Prime[i], {i, a, b}]
    // that maximizes |b-a|
    let sieve = primes::sieve(hi);

    let ps: Vec<usize> = (2..hi).filter(|&i| sieve[i]).collect();

    let mut best_total = 2;
    let mut best_range = 0;
    for i in 0..ps.len() {
        let mut j = i;
        let mut total = 0;
        while j < ps.len() && total < hi {
            if sieve[total] && j - i > best_range {
                best_total = total;
                best_range = j - i;
            }
            total += ps[j];
            j += 1;
        }
    }

    best_total
}
