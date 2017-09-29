use std::collections::HashMap;

// Compute the maximum sum of edge weights that can be removed while keeping the graph connected
pub fn solve(matrix: HashMap<(usize, usize), i32>) -> i32 {
    let total_weight = matrix.values().sum::<i32>() / 2; // the adjacency matrix is symmetric
    let num_vertices = 1 +
        matrix.keys().map(|&(i, _)| i).max().expect(
            "The matrix passed into solve() must be non-empty",
        );

    let mut edges: Vec<((usize, usize), i32)> = matrix.into_iter().collect();
    edges.sort_by_key(|&(_, weight)| weight);

    let mut uf = UnionFind::new(num_vertices);
    let mut span_weight = 0;
    for ((i, j), weight) in edges {
        if uf.find(i) != uf.find(j) {
            uf.union(i, j);
            span_weight += weight;
        }
    }

    total_weight - span_weight
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind { parent: (0..size).collect() }
    }

    fn find(&mut self, i: usize) -> usize {
        let parent = self.parent[i];
        if parent == i {
            return parent;
        }
        let ancestor = self.find(parent);
        self.parent[i] = ancestor;
        ancestor
    }
    fn union(&mut self, i: usize, j: usize) {
        let gi = self.find(i);
        let gj = self.find(j);
        if gi < gj {
            self.parent[gj] = gi;
        } else {
            self.parent[gi] = gj;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::path::Path;
    use std::fs::File;
    use std::io::{Error, Read};
    use std::str::FromStr;

    pub fn parse_matrix(path: &Path) -> Result<HashMap<(usize, usize), i32>, Error> {
        let mut fin = File::open(path)?;
        let mut s = String::new();
        fin.read_to_string(&mut s)?;

        let mut matrix: HashMap<(usize, usize), i32> = HashMap::new();
        for (i, line) in s.lines().enumerate() {
            for (j, tok) in line.split(',').enumerate() {
                if let Ok(weight) = i32::from_str(tok) {
                    matrix.insert((i, j), weight);
                }
            }
        }

        Ok(matrix)
    }

    #[test]
    fn small() {
        let matrix = parse_matrix(Path::new("data/p107_small.in")).expect("could not parse grid");
        assert_eq!(solve(matrix), 150);
    }

    #[test]
    fn main() {
        let matrix = parse_matrix(Path::new("data/p107_main.in")).expect("could not parse grid");
        assert_eq!(solve(matrix), 259679);
    }
}
