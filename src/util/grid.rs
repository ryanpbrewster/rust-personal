use std::ops::Index;

#[derive(Debug)]
pub struct Grid<T> {
    dim: (usize, usize),
    contents: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(dim: (usize, usize), contents: Vec<T>) -> Grid<T> {
        Grid {
            dim: dim,
            contents: contents,
        }
    }

    pub fn num_rows(&self) -> usize {
        self.dim.0
    }

    pub fn num_cols(&self) -> usize {
        self.dim.1
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let idx: usize = index.0 * self.num_rows() + index.1;
        &self.contents[idx]
    }
}
