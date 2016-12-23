// Sum[i, {i, 1, n}]
pub fn sum(n: u32) -> u32 {
    n*(n+1)/2
}

// Sum[i^2, {i, 1, n}]
pub fn sum_squares(n: u32) -> u32 {
    n*(n+1)*(2*n+1)/6
}


pub mod pythag {
    pub struct Triple(u32, u32, u32);
    impl Triple {
        pub fn new(a: u32, b: u32, c: u32) -> Triple {
            assert!(a < b && b < c && a*a + b*b == c*c);
            Triple(a, b, c)
        }
        pub fn check(a: u32, b: u32, c: u32) -> Option<Triple> {
            if a < b && b < c && a*a + b*b == c*c {
                Some(Triple(a, b, c))
            } else {
                None
            }
        }
    }
}
