/*
How many 20 digit numbers n (without any leading zero) exist such that no three
consecutive digits of n have a sum greater than 9?
*/

pub fn solve(num_digits: usize) -> u64 {
    let mut tallies: Vec<u64> = vec![0; 100];
    for d in 1..10 {
        tallies[d] = 1;
    }
    for _ in 1..num_digits {
        let mut tmp = vec![0; 100];
        for d0 in 0..10 {
            for d1 in 0..10 - d0 {
                for d2 in 0..10 - d0 - d1 {
                    tmp[10 * d1 + d2] += tallies[10 * d0 + d1];
                }
            }
        }
        tallies = tmp;
    }
    tallies.into_iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        assert_eq!(solve(1), 9);
        assert_eq!(solve(2), 45); // 10..18, 20..27, ... = 9 + 8 + ... + 1 = 45
    }

    #[test]
    fn main() {
        assert_eq!(solve(20), 378158756814587);
    }
}
