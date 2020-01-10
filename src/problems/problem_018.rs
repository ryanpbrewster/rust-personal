use crate::util::triangle::Triangle;
use std::cmp::max;

pub fn solve(t0: Triangle<i32>) -> i32 {
    let mut t = t0;
    for i in (0..t.height() - 1).rev() {
        for j in 0..i + 1 {
            t[(i, j)] += max(t[(i + 1, j)], t[(i + 1, j + 1)]);
        }
    }
    t[(0, 0)]
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn small() {
        // 3
        // 7 4
        // 2 4 6
        // 8 5 9 3
        let t = Triangle::new(vec![3, 7, 4, 2, 4, 6, 8, 5, 9, 3]);
        assert_eq!(solve(t), 23);
    }

    #[test]
    fn main() {
        let fin = File::open(Path::new("data/p018_main.in")).expect("couldn't open file");
        let t = Triangle::from_file(fin).expect("could not parse triangle");

        assert_eq!(solve(t), 1074);
    }
}
