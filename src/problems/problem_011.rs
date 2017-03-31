use util::grid::Grid;
use util::iter::Cross;

use std::fs::File;
use std::io::Read;
use std::path::Path;

#[test]
fn small() {
    let g = Grid::new((3, 3), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

    assert_eq!(solve(&g, 2), 8 * 9);
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
    assert_eq!(solve(&g, 4), 70600674);
}





#[derive(Clone, Copy)]
enum Direction {
    Down,
    Right,
    UpDiag,
    DownDiag,
}

impl Direction {
    fn all() -> [Direction; 4] {
        use self::Direction::*;
        [Down, Right, UpDiag, DownDiag]
    }
}


pub fn solve(grid: &Grid<i32>, n: usize) -> i32 {
    Direction::all()
        .iter()
        .flat_map(|&dir| analyze(grid, n, dir))
        .max()
        .unwrap()
        .clone()
}

fn analyze(grid: &Grid<i32>, n: usize, dir: Direction) -> Option<i32> {
    use self::Direction::*;
    let rows = match dir {
        Down => 0..grid.num_rows() - n + 1,
        Right => 0..grid.num_rows(),
        UpDiag => n - 1..grid.num_rows(),
        DownDiag => 0..grid.num_rows() - n + 1,
    };
    let cols = match dir {
        Down => 0..grid.num_cols(),
        Right => 0..grid.num_cols() - n + 1,
        UpDiag => n - 1..grid.num_cols() - n + 1,
        DownDiag => 0..grid.num_rows() - n + 1,
    };
    Cross::of(rows, cols)
        .map(|(i, j)| {
            (0..n)
                .map(|k| {
                    let idx = match dir {
                        Down => (i + k, j),
                        Right => (i, j + k),
                        UpDiag => (i - k, j + k),
                        DownDiag => (i + k, j + k),
                    };
                    grid[idx]
                })
                .fold(1, |a, b| a * b)
        })
        .max()

}
