/*
By using each of the digits from the set, {1, 2, 3, 4}, exactly once, and
making use of the four arithmetic operations (+, −, *, /) and
brackets/parentheses, it is possible to form different positive integer
targets.

For example,

8 = (4 * (1 + 3)) / 2
14 = 4 * (3 + 1 / 2)
19 = 4 * (2 + 3) − 1
36 = 3 * 4 * (2 + 1)

Note that concatenations of the digits, like 12 + 34, are not allowed.

Using the set, {1, 2, 3, 4}, it is possible to obtain thirty-one different
target numbers of which 36 is the maximum, and each of the numbers 1 to 28 can
be obtained before encountering the first non-expressible number.

Find the set of four distinct digits, a < b < c < d, for which the longest set
of consecutive positive integers, 1 to n, can be obtained, giving your answer
as a string: abcd.
*/

use std::collections::HashSet;

pub fn solve(size: usize) -> Vec<i32> {
    let digits: Vec<i32> = (0..10).collect();
    let mut ans = subsets(&digits, size)
        .into_iter()
        .max_by_key(|k| score(k))
        .expect("subsets guaranteed to be non-empty");
    ans.sort();
    ans
}

fn subsets<T: Clone>(universe: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 0 {
        vec![Vec::new()]
    } else if universe.is_empty() {
        Vec::new()
    } else {
        let tail = &universe[1..];
        let lose_it: Vec<Vec<T>> = subsets(tail, size);
        let use_it: Vec<Vec<T>> = {
            let x = &universe[0];
            let mut ys = subsets(&tail, size - 1);
            for y in ys.iter_mut() {
                y.push(x.clone())
            }
            ys
        };
        use_it.into_iter().chain(lose_it).collect()
    }

}

fn score(xs: &[i32]) -> u32 {
    let mut vis: HashSet<i32> = HashSet::new();
    let vs: Vec<f64> = xs.iter().map(|&x| x as f64).collect();
    explore(&mut vis, &vs);
    (1..).take_while(|&i| vis.contains(&i)).count() as u32
}


enum Op {
    Add,
    SubtractRight,
    SubtractLeft,
    Multiply,
    DivideRight,
    DivideLeft,
}
impl Op {
    fn all() -> [Op; 6] {
        use self::Op::*;
        [
            Add,
            SubtractLeft,
            SubtractRight,
            Multiply,
            DivideLeft,
            DivideRight,
        ]
    }
    fn apply(&self, x: f64, y: f64) -> f64 {
        use self::Op::*;
        match *self {
            Add => x + y,
            SubtractRight => x - y,
            SubtractLeft => y - x,
            Multiply => x * y,
            DivideRight => x / y,
            DivideLeft => y / x,
        }
    }
}
fn explore(vis: &mut HashSet<i32>, vs: &[f64]) {
    if vs.len() == 1 {
        let v = vs[0].round();
        if (v - vs[0]).abs() < 1e-2 {
            vis.insert(v as i32);
        }
    } else {
        for i in 0..vs.len() {
            for j in i + 1..vs.len() {
                let xs: Vec<f64> = vs.iter()
                    .cloned()
                    .enumerate()
                    .filter(|&(idx, _)| idx != i && idx != j)
                    .map(|(_, v)| v)
                    .collect();
                for op in Op::all().iter() {
                    let mut ys = xs.clone();
                    ys.push(op.apply(vs[i], vs[j]));
                    explore(vis, &ys);
                }

            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn subsets_small() {
        assert_eq!(
            subsets(&vec![1, 2, 3, 4], 2),
            vec![
                vec![2, 1],
                vec![3, 1],
                vec![4, 1],
                vec![3, 2],
                vec![4, 2],
                vec![4, 3],
            ]
        );
    }

    #[test]
    fn score_small() {
        assert_eq!(score(&vec![1, 2, 3, 4]), 28);
    }

    #[test]
    fn main() {
        assert_eq!(solve(4), vec![1, 2, 5, 8]);
    }
}
