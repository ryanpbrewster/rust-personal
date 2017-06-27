pub fn trial_division(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in (2..).take_while(|&i| i*i <= n) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
