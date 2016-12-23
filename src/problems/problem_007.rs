use util::primes::Primes;

pub fn solve(n: u32) -> u32 {
    Primes::all().nth((n-1) as usize).unwrap()
}
