pub fn solve(hi: u32) -> u32 {
    FibonacciElement::start()
        .into_iter()
        .take_while(|&n| n < hi)
        .filter(|&n| n % 2 == 0)
        .fold(0, |a, b| a + b)
}

struct FibonacciElement {
    cur: u32,
    prev: u32,
}

impl FibonacciElement {
    fn start() -> FibonacciElement {
        FibonacciElement { cur: 1, prev: 0 }
    }
}

impl Iterator for FibonacciElement {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.cur;
        self.cur += self.prev;
        self.prev = tmp;

        Some(tmp)
    }
}
