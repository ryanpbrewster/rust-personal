use test::Bencher;
use project_euler::util::prime;

#[bench]
fn all(bench: &mut Bencher) {
    const MAX: u32 = 1_000_000;
    bench.iter(|| {
        prime::all().take_while(|&n| n < MAX).sum::<u32>()
    })
}

