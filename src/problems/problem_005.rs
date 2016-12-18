extern crate num;

pub fn solve(n: u32) -> u32 {
    (1..n).fold(1, num::integer::lcm)
}
