use std::fs::File;
use std::io::Read;
use std::ops::Index;
use std::ops::IndexMut;
use std::str::FromStr;

#[derive(Clone, Debug)]
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

    pub fn size(&self) -> usize {
        self.dim.0 * self.dim.1
    }

    pub fn from_file(mut fin: File) -> Result<Grid<T>, String>
    where
        T: FromStr,
    {
        let mut s = String::new();
        fin.read_to_string(&mut s)
            .map_err(|_| "could not read file")?;

        let mut contents: Vec<T> = Vec::new();
        let lines: Vec<_> = s.lines().collect();

        let num_rows = lines.len();
        let num_cols = lines[0].split_whitespace().count();
        for line in lines {
            for tok in line.split_whitespace() {
                contents.push(
                    tok.parse::<T>()
                        .map_err(|_| format!("could not parse token: {}", tok))?,
                );
            }
        }

        Ok(Grid::new((num_rows, num_cols), contents))
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let idx: usize = index.0 * self.num_rows() + index.1;
        &self.contents[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut T {
        let idx: usize = index.0 * self.num_rows() + index.1;
        &mut self.contents[idx]
    }
}
