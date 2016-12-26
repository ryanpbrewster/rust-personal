use std::collections::HashSet;
use util::math::Digits;


pub fn solve() -> u32 {
    let valid_ranges = vec![(2..10, 1_000..10_000), (10..100, 100..1_000)];

    let mut products = HashSet::new();

    for (a_range, b_range) in valid_ranges {
        for a in a_range.clone() {
            for b in b_range.clone() {
                if !Digits::decimal(a).any(|da| Digits::decimal(b).any(|db| da == db)) {
                    let mut all_digits: Vec<u32> = Vec::new();
                    all_digits.extend(Digits::decimal(a));
                    all_digits.extend(Digits::decimal(b));
                    all_digits.extend(Digits::decimal(a * b));
                    all_digits.sort();
                    if all_digits == vec![1, 2, 3, 4, 5, 6, 7, 8, 9] {
                        products.insert(a * b);
                    }
                }
            }
        }
    }

    products.iter().sum()
}
