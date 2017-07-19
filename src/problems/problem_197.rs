/*
Given is the function f(x) = Floor[2^(30.403243784-x^2)] * 10^-9

The sequence u[n] is defined by u[0] = -1 and u[n+1] = f(u[n]).

Find u[n] + u[n+1] for n = 10^12.
Give your answer with 9 digits after the decimal point.
*/

/*
A few simple notes:
    f(x) = Floor[ 1.42*10^9 * 2^(-x^2) ] * 10^(-9)

Also note: u[n] converges to the very simple sequence
[1.029461842,0.681175875,1.029461842,0.681175875,...]

That is, it just jumps back and forth between those two values forever.
Thus, u[n] + u[n+1] == 1.029461842 + 0.681175875 == 1.710637717
*/

use num::Float;

pub fn solve() -> f64 {
    Useq(-1.0).skip(1_000).take(2).sum()
}

struct Useq(f64);
impl Iterator for Useq {
    type Item = f64;
    fn next(&mut self) -> Option<f64> {
        let x = self.0;
        self.0 = (1.42e9 * 2.0.powf(-x*x)).floor() * 1.0e-9;
        Some(x)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main() {
        let ans = format!("{:.9}", solve());
        assert_eq!(ans, "1.710637717");
    }
}
