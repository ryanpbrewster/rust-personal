pub struct Cross<A, B> {
    a: A,
    b: B,
    b0: B,
}

pub fn cross<A: Clone, B: Clone>(a: A, b: B) -> Cross<A, B> {
    Cross { a: a.clone(), b: b.clone(), b0: b.clone() }
}

impl <A, B> Iterator for Cross<A, B>
        where A: Iterator + Clone, B: Iterator + Clone {
    type Item = (A::Item, B::Item);
    fn next(&mut self) -> Option<(A::Item, B::Item)> {
        let bi = match self.b.next() {
            None => {
                self.a.next();
                self.b = self.b0.clone();
                self.b.next()
            },
            Some(val) => Some(val)
        };
        let ai = self.a.clone().next();
        match (ai, bi) {
            (Some(a_val), Some(b_val)) => Some((a_val, b_val)),
            _ => None
        }
    }
}
