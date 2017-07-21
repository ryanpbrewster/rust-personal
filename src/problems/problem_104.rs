use util::math::recurrence::{BigFibSeq, ModFibSeq};
use num::Float;

/*
Recall that F_k = (phi^k - phibar^k) / sqrt(5)
For large enough values of k, phibar^k approaches 0, so
F_k ~= phi^k / sqrt(5).
Suppose that F_k has d digits. To get the first 9 digits,
take floor(F_k / 10^(d - 8)).
Observe that log10(F_k / 10^(d-8))
  = log10(F_k) - (d - 8).
Recall that d = floor(log10(F_k)), so
log10(F_k) - floor(log10(F_k)) + 8
Observe that log10(F_k) = log10(phi^k / sqrt(5))
                        = k * log10(phi) - log10(5) / 2
*/

pub fn solve() -> usize {
    let phi: f64 = 0.5 * (1.0 + 5.0.sqrt());
    let mut candidates = ModFibSeq::new(1_000_000_000)
        .enumerate()
        .filter(|&(_, n)| is_pandigital(n.to_string().chars().take(9)))
        .map(|(k, _)| k);

    candidates
        .find(|&k| {
            let y = (k as f64) * phi.log10() - 0.5 * 5.0.log10();
            let x = 10f64.powf(y - y.floor() + 8.0);
            is_pandigital((x as u32).to_string().chars().take(9))
        })
        .unwrap()
}

fn is_pandigital<T>(digits: T) -> bool
where
    T: IntoIterator<Item = char>,
{
    let mut arr = vec![false; 10];
    arr[0] = true;

    let mut count = 0;
    for d in digits.into_iter() {
        let idx = d.to_digit(10).unwrap() as usize;
        if arr[idx] {
            return false;
        }
        arr[idx] = true;
        count += 1;
    }
    count == 9
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        assert_eq!(
            BigFibSeq::new()
                .enumerate()
                .find(|&(_, ref n)| {
                    is_pandigital(n.to_string().chars().rev().take(9))
                })
                .map(|(k, _)| k)
                .unwrap(),
            541
        );

        assert_eq!(
            BigFibSeq::new()
                .enumerate()
                .find(|&(_, ref n)| {
                    is_pandigital(n.to_str_radix(10).chars().take(9))
                })
                .map(|(k, _)| k)
                .unwrap(),
            2749
        );
    }

    #[test]
    fn main() {
        assert_eq!(solve(), 329468);
    }
}
