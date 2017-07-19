/*
Peter has nine four-sided (pyramidal) dice, each with faces numbered 1..4

Colin has six six-sided (cubic) dice, each with faces numbered 1..6

Peter and Colin roll their dice and compare totals: the highest total wins. The
result is a draw if the totals are equal.

What is the probability that Pyramidal Pete beats Cubic Colin? Give your answer
rounded to seven decimal places in the form 0.abcdefg
*/

pub fn solve(dice1: Vec<usize>, dice2: Vec<usize>) -> f64 {
    let dist1: Vec<f64> = distribution(dice1);
    let dist2: Vec<f64> = distribution(dice2);

    let mut p_win: f64 = 0.0;
    let mut cum_p2: f64 = 0.0;
    for (i, p1) in dist1.iter().enumerate() {
        p_win += p1 * cum_p2;
        if i < dist2.len() {
            cum_p2 += dist2[i];
        }
    }
    p_win
}

fn distribution(dice: Vec<usize>) -> Vec<f64> {
    let counts: Vec<u32> = dice.iter().fold(vec![1], |old, &n| {
        let mut updated = vec![0; old.len() + n as usize];
        for (i, &count) in old.iter().enumerate() {
            for j in 1 .. n + 1 {
                updated[i + j] += count;
            }
        }
        updated
    });
    let total: u32 = counts.iter().cloned().sum();
    counts.into_iter().map(|count| count as f64 / total as f64).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_approx_eq;

    #[test]
    fn distribution_small() {
        let dist = distribution(vec![6, 6]);
        assert_approx_eq!(dist[0], 0.0);
        assert_approx_eq!(dist[2], 1.0 / 36.0);
        assert_approx_eq!(dist[3], 2.0 / 36.0);
        assert_approx_eq!(dist[7], 6.0 / 36.0);
        assert_approx_eq!(dist[12], 1.0 / 36.0);
    }

    #[test]
    fn small() {
        assert_approx_eq!(solve(vec![6], vec![6]), 5.0 / 12.0);
    }

    #[test]
    fn main() {
        let ans = format!("{:.7}", solve(vec![4; 9], vec![6; 6]));
        assert_eq!(ans, "0.5731441");
    }
}
