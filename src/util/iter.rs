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


pub fn imerge<X: Iterator, Y: Iterator>(mut xs: X, mut ys: Y) -> IntersectionMerge<X, Y> {
    let (x, y) = (xs.next(), ys.next());
    IntersectionMerge {
        x: x,
        y: y,
        xs: xs,
        ys: ys,
    }
}

pub struct IntersectionMerge<X: Iterator, Y: Iterator> {
    x: Option<X::Item>,
    y: Option<Y::Item>,
    xs: X,
    ys: Y,
}

impl<T: Clone + Ord, X: Iterator<Item = T>, Y: Iterator<Item = T>> Iterator for IntersectionMerge<X, Y> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        while let (Some(x0), Some(y0)) = (self.x.clone(), self.y.clone()) {
            if x0 < y0 {
                self.x = self.xs.next();
            } else if y0 < x0 {
                self.y = self.ys.next();
            } else {
                break;
            }
        }
        match (self.x.clone(), self.y.clone()) {
            (Some(x0), Some(y0)) => {
                assert!(x0 == y0);
                let tmp = x0;
                self.x = self.xs.next();
                self.y = self.ys.next();
                Some(tmp)
            },
            (_, _) => None,
        }
    }
}
