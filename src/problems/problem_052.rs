use util::math::Digits;

pub fn solve(consecutive: usize) -> u32 {
    // Find the first number, `n`, such that [1*n, 2*n, ..., `consecutive` * n] all have the same digits
    (1..).find(|&n| {
        let mut ds: Vec<u32> = Digits::decimal(n).collect();
        ds.sort();
        (2..consecutive + 1).all(|k| {
            let mut kds: Vec<u32> = Digits::decimal(k as u32 * n).collect();
            kds.sort();
            kds == ds
        })
    }).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        assert_eq!(solve(1), 1);
        assert_eq!(solve(2), 125874);
    }

    #[test]
    fn main() {
        assert_eq!(solve(6), 142857);
    }
}
