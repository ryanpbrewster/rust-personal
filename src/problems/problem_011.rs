use util::grid::Grid;
use util::iter::Cross;

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


pub fn solve(grid: &Grid<i32>, n: usize) -> Option<i32> {
    Direction::all()
        .iter()
        .flat_map(|&dir| analyze(grid, n, dir))
        .max()
}

fn analyze(grid: &Grid<i32>, n: usize, dir: Direction) -> Option<i32> {
    use self::Direction::*;
    let rows = match dir {
        Down | DownDiag => 0..grid.num_rows() - n + 1,
        Right => 0..grid.num_rows(),
        UpDiag => n - 1..grid.num_rows(),
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn small() {
        let g = Grid::new((3, 3), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(solve(&g, 2).unwrap(), 8 * 9);
    }

    #[test]
    fn main() {
        let fin = File::open(Path::new("data/p011_main.in")).expect("couldn't open file");
        let g = Grid::from_file(fin).expect("could not parse grid");
        assert_eq!(solve(&g, 4).unwrap(), 70600674);
    }
}
