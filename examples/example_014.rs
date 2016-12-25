extern crate project_euler;

use project_euler::problems::problem_014;

fn main() {
    println!("{}", problem_014::solve_fast(1..1_000_000));
}
