use util::grid::Grid;
use util::iter::Cross;

pub fn solve(grid: &Grid<i32>, n: usize) -> i32 {
    let vert = analyze(grid, n, Direction::Down);
    let horiz = analyze(grid, n, Direction::Right);
    let down_diag = analyze(grid, n, Direction::DownDiag);
    let up_diag = analyze(grid, n, Direction::UpDiag);
    vec![vert.unwrap(), horiz.unwrap(), down_diag.unwrap(), up_diag.unwrap()]
        .iter()
        .max()
        .unwrap()
        .clone()
}

enum Direction {
    Down,
    Right,
    UpDiag,
    DownDiag,
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
