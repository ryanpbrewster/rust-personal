#![feature(test)]
extern crate test;

extern crate project_euler;

use test::Bencher;
use project_euler::util::prime;
use std::iter::Sum;

#[bench]
fn all(bench: &mut Bencher) {
    const MAX: u32 = 1_000_000;
    bench.iter(|| {
        prime::all().take_while(|&n| n < MAX).sum::<u32>()
    })
}

