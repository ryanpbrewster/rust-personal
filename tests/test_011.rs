extern crate project_euler;
use project_euler::problems::problem_011;
use project_euler::util::grid::Grid;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[test]
fn small() {
    let g = Grid::new((3, 3), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

    assert_eq!(problem_011::solve(&g, 2), 8 * 9);
}

#[test]
fn main() {
    let mut fin = match File::open(Path::new("tests/test_011.in")) {
        Err(e) => panic!("couldn't open file: {}", e),
        Ok(file) => file,
    };
    let mut s = String::new();
    match fin.read_to_string(&mut s) {
        Err(e) => panic!("couldn't read file: {}", e),
        Ok(_) => {}
    };

    let mut contents: Vec<i32> = Vec::new();
    let lines: Vec<_> = s.lines().collect();

    let num_rows = lines.len();
    let num_cols = lines[0].split(" ").count();
    for line in lines {
        for tok in line.split(" ") {
            match tok.parse::<i32>() {
                Ok(v) => contents.push(v),
                Err(e) => panic!("failed to parse: {}", e),
            };
        }
    }

    let g = Grid::new((num_rows, num_cols), contents);
    assert_eq!(problem_011::solve(&g, 4), 70600674);
}
