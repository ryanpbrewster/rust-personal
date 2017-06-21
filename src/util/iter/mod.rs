pub struct Cross<A, B> {
    a: A,
    b: B,
    b0: B,
}

impl<A, B> Cross<A, B>
where
    A: Clone,
    B: Clone,
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
where
    A: Iterator + Clone,
    B: Iterator + Clone,
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

impl<T: Ord, X: Iterator<Item = T>, Y: Iterator<Item = T>> Intersect<T, X, Y> {
    pub fn from(mut xs: X, mut ys: Y) -> Intersect<T, X, Y> {
        Intersect {
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





pub struct Group<S: Iterator> {
    source: S,
    prev: Option<S::Item>,
}

impl<S: Iterator> Group<S> {
    pub fn of(mut source: S) -> Group<S> {
        let prev = source.next();
        Group {
            source: source,
            prev: prev,
        }
    }
}

impl<S> Iterator for Group<S>
where
    S: Iterator,
    S::Item: Eq,
{
    type Item = (S::Item, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.prev.take().map(|x| {
            let mut count = 1;
            for cur in &mut self.source {
                if cur == x {
                    count += 1;
                } else {
                    self.prev = Some(cur);
                    break;
                }
            }
            (x, count)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn crossiter_finite() {
        let crossed: Vec<_> = Cross::of(1..3, 4..6).collect();
        assert_eq!(crossed, vec![(1, 4), (1, 5), (2, 4), (2, 5)]);
    }

    #[test]
    fn imerge_works() {
        let xs = vec![1, 3, 5, 7, 9];
        let ys = vec![2, 3, 4, 8, 9, 10];
        let mut iter = Intersect::from(xs.iter(), ys.iter());

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
