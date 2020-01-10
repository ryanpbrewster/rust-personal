use project_euler::util::prime;
use test::Bencher;

// test util::prime::iter::all_001k         ... bench:       6,991 ns/iter (+/- 4,169)
// test util::prime::iter::all_010k         ... bench:      66,074 ns/iter (+/- 33,707)
// test util::prime::iter::all_020k         ... bench:     129,324 ns/iter (+/- 179,753)
// test util::prime::iter::all_030k         ... bench:     164,342 ns/iter (+/- 199,892)
// test util::prime::iter::all_040k         ... bench:     309,033 ns/iter (+/- 190,585)
// test util::prime::iter::all_050k         ... bench:     337,767 ns/iter (+/- 166,300)
// test util::prime::iter::all_100k         ... bench:     726,656 ns/iter (+/- 350,477)

#[bench]
fn all_001k(bench: &mut Bencher) {
    all_param(bench, 1_000)
}
#[bench]
fn all_010k(bench: &mut Bencher) {
    all_param(bench, 10_000)
}
#[bench]
fn all_020k(bench: &mut Bencher) {
    all_param(bench, 20_000)
}
#[bench]
fn all_030k(bench: &mut Bencher) {
    all_param(bench, 30_000)
}
#[bench]
fn all_040k(bench: &mut Bencher) {
    all_param(bench, 40_000)
}
#[bench]
fn all_050k(bench: &mut Bencher) {
    all_param(bench, 50_000)
}
#[bench]
fn all_100k(bench: &mut Bencher) {
    all_param(bench, 100_000)
}

fn all_param(bench: &mut Bencher, bound: u32) {
    bench.iter(|| prime::all().take_while(|&n| n < bound).count())
}
