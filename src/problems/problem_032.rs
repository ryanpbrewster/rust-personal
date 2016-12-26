use std::collections::HashSet;


pub fn solve() -> u32 {
    let valid_ranges = vec![(1..10, 1_000..10_000), (10..100, 100..1_000)];

    let mut products = HashSet::new();

    for (a_range, b_range) in valid_ranges {
        for a in a_range.clone() {
            for b in b_range.clone() {
                if is_pandigital(a, b, a * b) {
                    products.insert(a * b);
                }
            }
        }
    }

    products.iter().sum()
}

fn is_pandigital(a: u32, b: u32, c: u32) -> bool {
    let mut digits: Vec<_> = vec![a, b, c]
        .into_iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .chars()
        .collect();

    digits.sort();

    digits.into_iter().collect::<String>() == "123456789"
}
