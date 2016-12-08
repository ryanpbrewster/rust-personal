extern crate project_euler;

use project_euler::util::primes;

fn main() {
	println!("{:?}", primes::sieve(20));
}
