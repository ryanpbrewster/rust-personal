extern crate project_euler;

use project_euler::util::coins;
use std::collections::HashSet;

fn main() {
    // An amazing Cribbage hand:
    println!("Can make 15 in {} ways",
             coins::ways_to_make(15, &vec![5, 5, 5, 5, 10]));

    // Making $1.00 from standard US coins
    let mut us_coins = HashSet::new();
    us_coins.insert(1);
    us_coins.insert(5);
    us_coins.insert(10);
    us_coins.insert(25);
    println!("Can make $1.00 in {} ways",
             coins::ways_to_make_with_replacement(100, &us_coins));
}
