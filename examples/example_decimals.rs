extern crate project_euler;

use project_euler::util::math::Decimals;

fn main() {
    for d in 2..100 {
        println!("1/{}  --> {:?}", d, Decimals::reciprocal(d).repeating());
    }
}
