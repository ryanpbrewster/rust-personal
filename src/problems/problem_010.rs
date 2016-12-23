use util::primes::Primes;

// Sum of all primes < hi
pub fn solve(hi: u32) -> u64 {
    Primes::all().take_while(|&p| p < hi).fold(0, |a, b| a + b as u64)
}
