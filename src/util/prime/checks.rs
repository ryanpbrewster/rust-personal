pub fn trial_division(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return true;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
