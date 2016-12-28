extern crate project_euler;

use project_euler::util::primes;

#[test]
fn primes_iter() {
    assert_eq!(primes::Primes::all().take(5).collect::<Vec<_>>(),
               vec![2, 3, 5, 7, 11]);
}

#[test]
fn primes_iter_correctness() {
    let sieve: Vec<bool> = primes::sieve(1_000);
    let ps: Vec<u32> = (2..1000).filter(|&i| sieve[i as usize]).collect();
    assert_eq!(primes::Primes::all().take_while(|&n| n < 1_000).collect::<Vec<_>>(), ps);
}

#[test]
fn primes_num_divisors() {
    // assert_eq!(primes::num_divisors(28), 6);
    assert_eq!(primes::num_divisors(36), 9);
}
