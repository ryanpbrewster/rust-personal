extern crate project_euler;

use project_euler::problems::*;

fn main() {
    println!("001: {:?}", problem_001::solve(1000));
    println!("002: {:?}", problem_002::solve(4_000_000));
    println!("003: {:?}", problem_003::solve(600851475143));
    println!("004: {:?}", problem_004::solve(3));
    println!("005: {:?}", problem_005::solve(20));
    println!("006: {:?}", problem_006::solve(100));
}
