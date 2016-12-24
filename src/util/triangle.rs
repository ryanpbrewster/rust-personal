// Represents a full lower-right triangle
// E.g.:
//   1
//   2 3
//   4 5 6
//   7 8 9 10

#[derive(Debug)]
pub struct Triangle<T> {
    contents: Vec<T>,
}

impl<T> Triangle<T> {
    pub fn new(contents: Vec<T>) -> Triangle<T> {
        Triangle { contents: contents }
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
