use util::math;

pub fn solve<T: Clone>(xs: Vec<T>, n: u64) -> Vec<T> {
    let n = n % math::factorial(xs.len() as u64);
    let mut xs = xs;
    let mut ys: Vec<T> = Vec::new();

    let len = xs.len();
    let mut cur: u64 = 0;

    for i in 0..len {
        let inc = math::factorial((len - i - 1) as u64);
        let k = (n - cur) / inc;
        cur += k * inc;
        ys.push(xs.remove(k as usize));
    }

    ys
}
#[test]
fn small() {
    assert_eq!(solve(vec![1, 2, 3], 0), vec![1, 2, 3]);
    assert_eq!(solve(vec![1, 2, 3], 2), vec![2, 1, 3]);
    assert_eq!(solve(vec![1, 2, 3], 6), vec![1, 2, 3]);
}

#[test]
fn main() {
    assert_eq!(solve((0..10).collect(), 1_000_000 - 1),
               vec![2, 7, 8, 3, 9, 1, 5, 4, 6, 0]);
}
