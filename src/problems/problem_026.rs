use util::math::Decimals;

pub fn solve(hi: u32) -> u32 {
    (2..hi).max_by_key(|&i| Decimals::reciprocal(i).repeating().1.len()).unwrap()
}
