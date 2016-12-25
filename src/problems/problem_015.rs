use util::math;

pub fn solve(height: u64, width: u64) -> u64 {
    math::choose(height + width, width)
}
