extern crate project_euler;

use project_euler::util::triangle::Triangle;
use project_euler::util::triangle::Levels;

fn main() {
    let t = Triangle::new(vec![1, 2, 3, 4, 5, 6]);
    println!("{:?}", t);

    for lvl in Levels::from(&t) {
        println!("{:?}", lvl);
    }
}
