use crate::util::math::pythag;

// Find the value, p <= hi, for which the most pythagorean triples
// exist that satisfy a + b + c == p
pub fn solve(hi: usize) -> usize {
    let mut solution_count = vec![0; hi + 1];
    let mut q = vec![pythag::Triple::root()];

    while let Some(trip) = q.pop() {
        let p = trip.sum() as usize;
        if p < hi {
            for lambda in 1..1 + hi / p {
                solution_count[lambda * p] += 1;
            }
            for child in trip.branch() {
                q.push(child);
            }
        }
    }

    solution_count
        .iter()
        .enumerate()
        .max_by_key(|&(_, count)| count)
        .map(|(idx, _)| idx)
        .unwrap()
}

#[test]
fn main() {
    assert_eq!(solve(1000), 840);
}
