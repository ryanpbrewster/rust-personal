extern crate project_euler;

use project_euler::util::primes;

fn main() {
    println!("{}", primes::Primes::all().nth(1_000_000).unwrap());
}
