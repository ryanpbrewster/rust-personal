#[derive(PartialEq, PartialOrd, Ord, Debug, Eq, Clone, Copy)]
pub struct Triple {
    pub a: u32,
    pub b: u32,
    pub c: u32
}

impl Triple {
    pub fn root() -> Triple {
        Triple { a: 3, b: 4, c: 5 }
    }
    pub fn new(a: u32, b: u32, c: u32) -> Triple {
        assert!(a < b && b < c && a * a + b * b == c * c);
        Triple { a: a, b: b, c: c }
    }
    pub fn check(a: u32, b: u32, c: u32) -> Option<Triple> {
        if a < b && b < c && a * a + b * b == c * c {
            Some(Triple { a: a, b: b, c: c })
        } else {
            None
        }
    }

    pub fn sum(&self) -> u32 {
        self.a + self.b + self.c
    }
    pub fn product(&self) -> u32 {
        self.a * self.b * self.c
    }
    pub fn branch(&self) -> Vec<Triple> {
        vec![
            Triple::new(2 * self.c - 2 * self.a + self.b, 2 * self.c - self.a + 2 * self.b, 3 * self.c - 2 * self.a + 2 * self.b),
            Triple::new(2 * self.c + 2 * self.a + self.b, 2 * self.c + self.a + 2 * self.b, 3 * self.c + 2 * self.a + 2 * self.b),
            Triple::new(2 * self.c + self.a - 2 * self.b, 2 * self.c + 2 * self.a - self.b, 3 * self.c + 2 * self.a - 2 * self.b),
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pythag_triple() {
        assert!(Triple::check(1, 2, 3).is_none());
        assert!(Triple::check(3, 4, 5).is_some());
    }

    #[test]
    fn pythag_triple_tree_branch() {
        assert_eq!(
            Triple::new(3, 4, 5).branch(),
            [
                Triple::new(8, 15, 17),
                Triple::new(20, 21, 29),
                Triple::new(5, 12, 13),
            ]
        );
    }
}
