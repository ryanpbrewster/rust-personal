// Represents a full lower-right triangle
// E.g.:
//   1
//   2 3
//   4 5 6
//   7 8 9 10

use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug)]
pub struct Triangle<T> {
    contents: Vec<T>,
}

impl<T> Triangle<T> {
    pub fn new(contents: Vec<T>) -> Triangle<T> {
        Triangle { contents: contents }
    }
    pub fn height(&self) -> usize {
        let n = self.contents.len();
        // Observe that h*(h+1)/2 == n  --> h = 1/2 * (sqrt(8 * n + 1) - 1)
        (0.5 * ((8.0 * n as f64 + 1.0).sqrt() - 1.0)).round() as usize
    }
}

impl<T: Sized> Index<(usize, usize)> for Triangle<T> {
    type Output = T;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.contents[idx(i) + j]
    }
}

impl<T: Sized> IndexMut<(usize, usize)> for Triangle<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        &mut self.contents[idx(i) + j]
    }
}

#[derive(Debug)]
pub struct Levels<'a, T: 'a> {
    source: &'a Triangle<T>,
    cur_level: usize,
}

impl<'a, T> Levels<'a, T> {
    pub fn from(t: &'a Triangle<T>) -> Levels<'a, T> {
        Levels {
            source: t,
            cur_level: 0,
        }
    }
}


fn idx(lvl: usize) -> usize {
    lvl * (lvl + 1) / 2
}
impl<'a, T> Iterator for Levels<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        let start = idx(self.cur_level);
        let end = idx(self.cur_level + 1);

        if end > self.source.contents.len() {
            None
        } else {
            self.cur_level += 1;
            Some(&self.source.contents[start..end])
        }
    }
}
