extern crate project_euler;

use project_euler::util::primes;

#[test]
fn primes_iter() {
    assert_eq!(primes::Primes::all().take(5).collect::<Vec<_>>(),
               vec![2, 3, 5, 7, 11]);
}

#[test]
fn primes_iter_correctness() {
    assert_eq!(primes::Primes::all().take_while(|&n| n < 1_000).collect::<Vec<_>>(),
               primes::sieve(1_000));
}
