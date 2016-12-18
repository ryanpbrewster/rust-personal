use util::primes;

pub fn solve(n: u32) -> u32 {
    primes::primes().nth((n-1) as usize).unwrap()
}
