use util::grid::Grid;
use petgraph::graph::Graph;
use petgraph::algo;

pub fn solve(weight_grid: &Grid<u32>) -> u32 {
    let mut g = Graph::<u32, u32>::new();
    let n_rows = weight_grid.num_rows();
    let n_cols = weight_grid.num_cols();

    // Construct two pseudo-nodes, one on the left and one on the right
    let left = g.add_node(0);
    let right = g.add_node(0);

    let node_grid = Grid::new(
        (n_rows, n_cols),
        (0..n_rows * n_cols).map(|_| g.add_node(0)).collect()
    );

    // The left pseudo-node connects to the top-left cell
    g.add_edge(left, node_grid[(0, 0)], weight_grid[(0, 0)]);
    // and the right psuedo-node to the bottom-right cell
    g.add_edge(node_grid[(n_rows - 1, n_cols - 1)], right, 0);

    // Introduce the inter-cell edges (up, down, left, right).
    for i in 0..n_rows {
        for j in 0..n_cols {
            if i > 0 {
                g.add_edge(node_grid[(i, j)], node_grid[(i - 1, j)], weight_grid[(i - 1, j)]); // up
                g.add_edge(node_grid[(i - 1, j)], node_grid[(i, j)], weight_grid[(i, j)]); // down
            }
            if j > 0 {
                g.add_edge(node_grid[(i, j - 1)], node_grid[(i, j)], weight_grid[(i, j)]); // right
                g.add_edge(node_grid[(i, j)], node_grid[(i, j - 1)], weight_grid[(i, j - 1)]); // left
            }
        }
    }

    let shortest_paths = algo::dijkstra(&g, left, Some(right), |e| *e.weight());
    *shortest_paths.get(&right).expect("dijkstra should calculate costs for every node in the graph")
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
        let fin = File::open(Path::new("data/p083_small.in")).expect("couldn't open file");
        let g = Grid::from_file(fin).expect("could not parse grid");
        assert_eq!(solve(&g), 2297);
    }

    #[test]
    fn main() {
        let fin = File::open(Path::new("data/p083_main.in")).expect("couldn't open file");
        let g = Grid::from_file(fin).expect("could not parse grid");
        assert_eq!(solve(&g), 425185);
    }
}
