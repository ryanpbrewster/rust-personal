extern crate project_euler;

use project_euler::util::primes;

#[test]
fn primes_iter() {
	assert_eq!(
		primes::primes().take(5).collect::<Vec<_>>(),
		vec![2, 3, 5, 7, 11]);
}
