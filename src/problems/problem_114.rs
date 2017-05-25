pub fn solve() -> usize {
    f(50)
}

fn f(n: usize) -> usize {
    let mut blacks: Vec<usize> = vec![1];
    let mut reds: Vec<usize> = vec![0];
    for i in 1..n+1 {
        let bi = blacks[i-1] + reds[i-1];
        let ri = (3..i+1).map(|r| blacks[i-r]).sum();

        blacks.push(bi);
        reds.push(ri);
    }
    blacks[n] + reds[n]
}

#[cfg(test)]
mod test {
    use super::*;

    // Brute-force compute the number of arrangements starting with a black block
    fn bf_black(n: usize) -> usize {
        if n == 0 {
            1
        } else {
            bf_red(n-1) + bf_black(n-1)
        }
    }

    // Brute-force compute the number of arrangements starting with a red block
    fn bf_red(n: usize) -> usize {
        (3..n+1).map(|r| bf_black(n - r)).sum()
    }

    #[test]
    fn tiny() {
        assert_eq!(bf_black(0), 1);
        assert_eq!(bf_black(3), 1);
        assert_eq!(bf_red(0), 0);
        assert_eq!(bf_red(3), 1);
    }

    #[test]
    fn small() {
        assert_eq!(bf_black(7) + bf_red(7), 17);
        assert_eq!(f(7), 17);
    }

    #[test]
    fn main() {
        assert_eq!(f(50), 16475640049);
    }
}
