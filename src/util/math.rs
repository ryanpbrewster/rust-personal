// Sum[i, {i, 1, n}]
pub fn sum(n: u32) -> u32 {
    n*(n+1)/2
}

// Sum[i^2, {i, 1, n}]
pub fn sum_squares(n: u32) -> u32 {
    n*(n+1)*(2*n+1)/6
}
