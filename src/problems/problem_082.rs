use util::grid::Grid;
use petgraph::graph::Graph;
use petgraph::algo;

pub fn solve(weight_grid: &Grid<u32>) -> u32 {
    let mut g = Graph::<u32, u32>::new();

    // Construct two pseudo-nodes, one on the left and one on the right
    let left = g.add_node(0);
    let right = g.add_node(0);

    let node_grid = Grid::new(
        (weight_grid.num_rows(), weight_grid.num_cols()),
        (0..weight_grid.size()).map(|_| g.add_node(0)).collect());

    // The left pseudo-node connects to every cell on the left-most column,
    // and the right to the right-most column.
    for i in 0 .. weight_grid.num_rows() {
        g.add_edge(left, node_grid[(i, 0)], weight_grid[(i, 0)]);
        g.add_edge(node_grid[(i, weight_grid.num_cols() - 1)], right, 0);
    }

    // Introduce the inter-cell edges (up, down, and right).
    for i in 0..weight_grid.num_rows() {
        for j in 0..weight_grid.num_cols() {
            if i > 0 {
                g.add_edge(node_grid[(i, j)], node_grid[(i - 1, j)], weight_grid[(i - 1, j)]); // up
                g.add_edge(node_grid[(i - 1, j)], node_grid[(i, j)], weight_grid[(i, j)]); // down
            }
            if j > 0 {
                g.add_edge(node_grid[(i, j - 1)], node_grid[(i, j)], weight_grid[(i, j)]); // right
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

        assert_eq!(solve(&g), 1 + 2 + 3);
    }

    #[test]
    fn small() {
        let fin = File::open(Path::new("data/p082_small.in")).expect("couldn't open file");
        let g = Grid::from_file(fin).expect("could not parse grid");
        assert_eq!(solve(&g), 994);
    }

    #[test]
    fn main() {
        let fin = File::open(Path::new("data/p082_main.in")).expect("couldn't open file");
        let g = Grid::from_file(fin).expect("could not parse grid");
        assert_eq!(solve(&g), 260324);
    }
}
