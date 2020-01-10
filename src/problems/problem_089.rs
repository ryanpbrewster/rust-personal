/*
For a number written in Roman numerals to be considered valid there are basic
rules which must be followed. Even though the rules allow some numbers to be
expressed in more than one way there is always a "best" way of writing a
particular number.

For example, it would appear that there are at least six ways of writing the
number sixteen:

IIIIIIIIIIIIIIII
VIIIIIIIIIII
VVIIIIII
XIIIIII
VVVI
XVI

However, according to the rules only XIIIIII and XVI are valid, and the last
example is considered to be the most efficient, as it uses the least number of
numerals.

The 11K text file, roman.txt (right click and 'Save Link/Target As...'),
contains one thousand numbers written in valid, but not necessarily minimal,
Roman numerals; see About... Roman Numerals for the definitive rules for this
problem.

Find the number of characters saved by writing each of these in their minimal
form.
*/

pub fn solve(numeral: String) -> usize {
    let n = decode_numeral(&numeral);
    let optimal = encode_numeral(n);
    numeral.len() - optimal.len()
}

fn decode_numeral(numeral: &str) -> u32 {
    let components: Vec<u32> = numeral.chars().map(|ch| component_value(ch)).collect();
    let mut n = 0;
    let mut i = 0;
    while i < components.len() {
        if i + 1 < components.len() && components[i] < components[i + 1] {
            n += components[i + 1] - components[i];
            i += 2;
        } else {
            n += components[i];
            i += 1;
        }
    }
    n
}

const NUMERAL_BREAKPOINTS: [(u32, &'static str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];
fn encode_numeral(n: u32) -> String {
    let mut n = n;
    let mut str = String::new();
    while let Some(c) = NUMERAL_BREAKPOINTS.iter().find(|&c| c.0 <= n) {
        n -= c.0;
        str.push_str(c.1);
    }
    str
}

fn component_value(ch: char) -> u32 {
    match ch {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => panic!("Bad numeral component: {}", ch),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{Rng, SeedableRng, XorShiftRng};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    #[test]
    fn encode_numeral_manual() {
        assert_eq!(encode_numeral(1), "I");
        assert_eq!(encode_numeral(3), "III");
        assert_eq!(encode_numeral(4), "IV");
        assert_eq!(encode_numeral(5), "V");
        assert_eq!(encode_numeral(8), "VIII");
        assert_eq!(encode_numeral(9), "IX");
        assert_eq!(encode_numeral(10), "X");
        assert_eq!(encode_numeral(11), "XI");
        assert_eq!(encode_numeral(14), "XIV");
        assert_eq!(encode_numeral(15), "XV");
        assert_eq!(encode_numeral(16), "XVI");
        assert_eq!(encode_numeral(29), "XXIX");
        assert_eq!(encode_numeral(34), "XXXIV");
        assert_eq!(encode_numeral(49), "XLIX");
        assert_eq!(encode_numeral(50), "L");
        assert_eq!(encode_numeral(54), "LIV");
        assert_eq!(encode_numeral(59), "LIX");
        assert_eq!(encode_numeral(64), "LXIV");
        assert_eq!(encode_numeral(90), "XC");
        assert_eq!(encode_numeral(91), "XCI");
        assert_eq!(encode_numeral(94), "XCIV");
        assert_eq!(encode_numeral(95), "XCV");
        assert_eq!(encode_numeral(97), "XCVII");
        assert_eq!(encode_numeral(99), "XCIX");
        assert_eq!(encode_numeral(490), "CDXC");
    }

    #[test]
    fn encode_decode_bijection() {
        let mut prng = XorShiftRng::from_seed([42, 42, 42, 42]);
        for _ in 0..1000 {
            let n = prng.gen_range(1, 10_000);
            assert_eq!(decode_numeral(&encode_numeral(n)), n);
        }
    }

    #[test]
    fn main() {
        let fin = File::open(Path::new("data/p089_main.in")).expect("couldn't open file");
        let numerals = BufReader::new(fin)
            .lines()
            .collect::<Result<Vec<String>, _>>()
            .expect("couldn't read file");
        let ans: usize = numerals.into_iter().map(|num| solve(num)).sum();
        assert_eq!(ans, 743);
    }
}
