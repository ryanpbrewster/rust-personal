fn main() {
    println!("{}", solve(1000));
}

fn solve(n: i32) -> i32 {
    (1..n).filter(|n| n%3 == 0 || n%5 == 0).fold(0, |acc, n| acc + n)
}
