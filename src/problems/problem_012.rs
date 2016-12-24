use util::primes;

// Find the first triangle number with > `lower_bound` divisors
pub fn solve(lower_bound: u32) -> u32 {
    (1..)
        .map(|i| i * (i + 1) / 2)
        .find(|&n| primes::num_divisors(n as u64) > lower_bound)
        .expect("couldn't find any triangle number with > lower_bound divisors")
}
