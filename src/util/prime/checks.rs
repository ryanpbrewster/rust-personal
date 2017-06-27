pub fn trial_division(n: u64) -> bool {
    // all:
    //   1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29
    // wheel 2:
    //   1   3   5   7   9    11    13    15    17    19    21    23    25    27    29
    // wheel 2 + 3:
    //   1       5   7        11    13          17    19          23    25          29
    // wheel 2 + 3 + 5:
    //   1           7        11    13          17    19          23                29
    let spokes = [6, 4, 2, 4, 2, 4, 6, 2];
    let wheel = spokes.iter().cycle().scan(1, |acc, &d| {
        *acc += d;
        Some(*acc)
    });

    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 || n == 5 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 {
        return false;
    }

    for i in wheel.take_while(|&i| i * i <= n) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    fn divisor(n: u64) -> u64 {
        assert!(n >= 2);
        (2..).find(|&i| n % i == 0).unwrap()
    }

    #[test]
    fn small() {
        assert_eq!(trial_division(0), false);
        assert_eq!(trial_division(1), false);
        assert_eq!(trial_division(2), true);
        assert_eq!(trial_division(3), true);
        assert_eq!(trial_division(4), false);
        assert_eq!(trial_division(5), true);
        assert_eq!(trial_division(6), false);
        assert_eq!(trial_division(7), true);
        assert_eq!(trial_division(8), false);
        assert_eq!(trial_division(9), false);
    }

    #[test]
    fn medium() {
        for n in 2..10_000 {
            assert_eq!(
                trial_division(n),
                divisor(n) == n,
                "disagree whether {} is prime, smallest divisor = {}",
                n,
                divisor(n)
            );
        }
    }
}
