extern crate project_euler;

use project_euler::util::prime::Primes;

fn main() {
    println!("{}", Primes::all().nth(1_000_000).unwrap());
}
