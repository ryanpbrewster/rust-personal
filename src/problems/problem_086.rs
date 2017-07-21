/*
A spider, S, sits in one corner of a cuboid room, measuring 6 by 5 by 3, and a fly, F, sits in the opposite corner. By travelling on the surfaces of the room the shortest "straight line" distance from S to F is 10 and the path is shown on the diagram.

However, there are up to three "shortest" path candidates for any given cuboid and the shortest route doesn't always have integer length.

It can be shown that there are exactly 2060 distinct cuboids, ignoring rotations, with integer dimensions, up to a maximum size of M by M by M, for which the shortest route has integer length when M = 100. This is the least value of M for which the number of solutions first exceeds two thousand; the number of solutions when M = 99 is 1975.

Find the least value of M such that the number of solutions first exceeds one million.
*/

/*
"Solutions" are triples (w, l, h) where w >= l >= h and:
  - (l, w + h)
  - (w, l + h)
are the short legs of Pythagorean triples
*/

pub fn solve(threshold: u32) -> u32 {
    let mut count = 0;
    for w in 2.. {
        for lh in 2..2 * w + 1 {
            if is_square(w * w + lh * lh) {
                // How many ways can we create (w, l, h) such that w >= l >= h and l + h == lh
                let h_lo = if lh > w { lh - w } else { 1 };
                let h_hi = lh / 2;
                count += h_hi - h_lo + 1;
            }
        }
        if count > threshold {
            return w;
        }
    }
    panic!("Unreachable")
}

fn is_square(n: u32) -> bool {
    let r = (n as f64).sqrt().round() as u32;
    r * r == n
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tiny() {
        assert_eq!(solve(10), 9);
    }

    #[test]
    fn small() {
        assert_eq!(solve(2000), 100);
    }

    #[test]
    fn main() {
        assert_eq!(solve(1_000_000), 1818);
    }
}
