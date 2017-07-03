pub fn trial_division(n: u64) -> bool {
    use util::prime::wheel::Wheel;
    if n == 2 || n == 3 || n == 5 || n == 7 {
        return true;
    }
    if n < 2 || n % 2 == 0 || n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
        return false;
    }

    !Wheel::new().take_while(|&i| i * i <= n).any(|i| n % i == 0)
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
