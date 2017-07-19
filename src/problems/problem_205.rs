use num::{BigRational, BigInt};
use num::rational::Ratio;
use num::{One, Zero};

/*
Peter has nine four-sided (pyramidal) dice, each with faces numbered 1..4

Colin has six six-sided (cubic) dice, each with faces numbered 1..6

Peter and Colin roll their dice and compare totals: the highest total wins. The
result is a draw if the totals are equal.

What is the probability that Pyramidal Pete beats Cubic Colin? Give your answer
rounded to seven decimal places in the form 0.abcdefg
*/

pub fn solve(dice1: Vec<u32>, dice2: Vec<u32>) -> BigRational {
    let dist1: Vec<BigRational> = distribution(dice1);
    let dist2: Vec<BigRational> = distribution(dice2);

    let mut p_win: BigRational = Ratio::zero();
    let mut cum_p2: BigRational = Ratio::zero();
    for (i, p1) in dist1.iter().enumerate() {
        p_win = &p_win + p1 * &cum_p2;
        if i < dist2.len() {
            cum_p2 = &cum_p2 + &dist2[i];
        }
    }
    p_win
}

fn distribution(dice: Vec<u32>) -> Vec<BigRational> {
    let counts: Vec<BigInt> = dice.iter().fold(vec![BigInt::one()], |old, &n| {
        let mut updated = vec![BigInt::zero(); old.len() + n as usize];
        for (i, count) in old.iter().enumerate() {
            for j in 1 .. n + 1 {
                updated[i + j as usize] = &updated[i + j as usize] + count;
            }
        }
        updated
    });
    let total: BigInt = counts.iter().cloned().fold(BigInt::zero(), |a, b| a + b);
    counts.into_iter().map(|count| Ratio::new(count, total.clone())).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use num::ToPrimitive;

    fn approx(x: BigRational) -> f64 {
        x.numer().to_f64().unwrap() / x.denom().to_f64().unwrap()
    }

    #[test]
    fn distribution_small() {
        let dist = distribution(vec![6, 6]);
        assert_eq!(dist[0], Ratio::zero());
        assert_eq!(dist[2],  Ratio::new(BigInt::from(1), BigInt::from(36)));
        assert_eq!(dist[3],  Ratio::new(BigInt::from(2), BigInt::from(36)));
        assert_eq!(dist[7],  Ratio::new(BigInt::from(6), BigInt::from(36)));
        assert_eq!(dist[12], Ratio::new(BigInt::from(1), BigInt::from(36)));
    }

    #[test]
    fn small() {
        assert_eq!(approx(solve(vec![6], vec![6])), 5.0 / 12.0);
    }

    #[test]
    fn main() {
        let ans = format!("{:.7}", approx(solve(vec![4; 9], vec![6; 6])));
        assert_eq!(ans, "0.5731441");
    }
}
