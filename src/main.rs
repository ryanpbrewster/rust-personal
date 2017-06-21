extern crate project_euler;

use project_euler::util::prime;

fn main() {
    println!("{:?}", prime::sieve(20));
}
