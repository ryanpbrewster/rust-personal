use util::primes;

pub fn solve(n: u64) -> u64 {
    primes::factors(n).into_iter().last().expect("no factors!")
}
