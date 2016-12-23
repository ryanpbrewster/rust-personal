pub fn solve(numbers: &Vec<i32>, n: usize) -> i64 {
    numbers.windows(n)
        .map(|chunk| chunk.iter().fold(1i64, |a, &b| a * b as i64))
        .max()
        .unwrap()
}
