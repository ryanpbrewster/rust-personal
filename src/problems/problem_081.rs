use std::cmp;
use util::grid::Grid;

pub fn solve(grid: &Grid<u32>) -> u32 {
    let mut dp = grid.clone();
    for j in 1..grid.num_cols() {
        dp[(0, j)] += dp[(0, j - 1)];
    }
    for i in 1..grid.num_rows() {
        dp[(i, 0)] += dp[(i - 1, 0)];
        for j in 1..grid.num_cols() {
            dp[(i, j)] += cmp::min(dp[(i - 1, j)], dp[(i, j - 1)]);
        }
    }
    dp[(grid.num_rows() - 1, grid.num_cols() - 1)]
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn tiny() {
        let g = Grid::new((3, 3), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(solve(&g), 1 + 2 + 3 + 6 + 9);
    }

    #[test]
    fn small() {
        let fin = File::open(Path::new("data/p081_small.in")).expect("couldn't open file");
        let g = Grid::from_file(fin).expect("could not parse grid");
        assert_eq!(solve(&g), 2427);
    }

    #[test]
    fn main() {
        let fin = File::open(Path::new("data/p081_main.in")).expect("couldn't open file");
        let g = Grid::from_file(fin).expect("could not parse grid");
        assert_eq!(solve(&g), 427337);
    }
}
