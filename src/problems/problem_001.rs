pub fn solve(n: u32) -> u32 {
    (1..n).filter(|n| n % 3 == 0 || n % 5 == 0).fold(0, |a, b| a + b)
}
