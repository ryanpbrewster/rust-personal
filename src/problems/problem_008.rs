pub fn solve(numbers: Vec<i32>, n: usize) -> i32 {
    numbers.iter().chunks(n).map(|chunk| chunk.iter().fold(1, |a, b| a*b)).max().unwrap_or(-1)
}
