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
