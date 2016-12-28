pub struct Cross<A, B> {
    a: A,
    b: B,
    b0: B,
}

impl<A, B> Cross<A, B>
    where A: Clone,
          B: Clone
{
    pub fn of(a: A, b: B) -> Cross<A, B> {
        Cross {
            a: a.clone(),
            b: b.clone(),
            b0: b.clone(),
        }
    }
}

impl<A, B> Iterator for Cross<A, B>
    where A: Iterator + Clone,
          B: Iterator + Clone
{
    type Item = (A::Item, B::Item);
    fn next(&mut self) -> Option<(A::Item, B::Item)> {
        let bi = match self.b.next() {
            None => {
                self.a.next();
                self.b = self.b0.clone();
                self.b.next()
            }
            Some(val) => Some(val),
        };
        let ai = self.a.clone().next();
        match (ai, bi) {
            (Some(a_val), Some(b_val)) => Some((a_val, b_val)),
            _ => None,
        }
    }
}


fn both<T>(x: Option<T>, y: Option<T>) -> Option<(T, T)> {
    match (x, y) {
        (Some(x0), Some(y0)) => Some((x0, y0)),
        (_, _) => None,
    }
}
pub struct Intersect<T, X, Y> {
    cur: Option<(T, T)>,
    xs: X,
    ys: Y,
}

impl <T: Ord, X: Iterator<Item = T>, Y: Iterator<Item = T>> Intersect<T, X, Y> {
    pub fn from(mut xs: X, mut ys: Y) -> Intersect<T, X, Y> {
        Intersect{
            cur: both(xs.next(), ys.next()),
            xs: xs,
            ys: ys,
        }
}

}

impl<T: Ord, X: Iterator<Item = T>, Y: Iterator<Item = T>> Iterator for Intersect<T, X, Y> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((x0, y0)) = self.cur.take() {
            if x0 < y0 {
                self.cur = self.xs.next().map(|x1| (x1, y0));
            } else if y0 < x0 {
                self.cur = self.ys.next().map(|y1| (x0, y1));
            } else {
                self.cur = both(self.xs.next(), self.ys.next());
                return Some(x0);
            }
        }
        None
    }
}
