use util::math;

pub fn solve(n: u32) -> u32 {
    let s = math::sum(n);
    let s_sq = math::sum_squares(n);

    s * s - s_sq
}
