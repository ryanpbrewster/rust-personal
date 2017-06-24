/*
Find the element with the largest value in a list of base-exponent pairs
*/

pub fn solve(be_pairs: &[(u32, u32)]) -> Option<usize> {
    be_pairs
        .iter()
        .enumerate()
        .max_by_key(|&(_, &(b, e))| witness(b, e))
        .map(|(i, _)| i)
}

fn witness(b: u32, e: u32) -> u64 {
    let x = (e as f64) * (b as f64).ln();
    x as u64
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn small() {
        assert_eq!(solve(vec![(2, 11), (3, 7)].as_slice()).unwrap(), 1);
    }

    #[test]
    fn main() {
        let mut fin = File::open(Path::new("data/p099_main.in")).expect("couldn't open file");
        let mut s = String::new();
        fin.read_to_string(&mut s).expect(
            "could not read file into memory",
        );
        let be_pairs: Vec<(u32, u32)> = s.lines()
            .map(|line| {
                let vs: Vec<u32> = line.split_whitespace()
                    .map(|tok| tok.parse::<u32>().unwrap())
                    .collect();
                (vs[0], vs[1])
            })
            .collect();

        // Project Euler is doing 1-indexing, so add 1.
        assert_eq!(1 + solve(be_pairs.as_slice()).unwrap(), 709);
    }
}
