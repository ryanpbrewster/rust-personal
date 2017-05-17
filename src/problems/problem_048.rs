const MOD: u64 = 10_000_000_000;
pub fn solve(n: u64) -> u64 {
    // Sum[i^i, {i, 1, n}] % 10^10
    (1..n + 1).map(|i| pow_mod(i, i, MOD)).sum::<u64>() % MOD
}

fn pow_mod(b: u64, e: u64, m: u64) -> u64 {
    let mut n = 1;
    for _ in 0..e {
        n = (n * b) % m;
    }
    n
}

#[test]
fn small() {
    assert_eq!(solve(10), 0405071317);
}

#[test]
fn main() {
    assert_eq!(solve(1_000), 9110846700);
}
