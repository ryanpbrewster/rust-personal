use util::iter::Intersect;

pub fn solve(lo: u32) -> u32 {
    Intersect::from((1..).map(|n| n * (3 * n - 1) / 2),
                    (1..).map(|n| n * (2 * n - 1)))
        .skip_while(|&n| n <= lo)
        .next()
        .unwrap()
}
