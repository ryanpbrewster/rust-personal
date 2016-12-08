pub fn solve(hi: u32) -> u32 {
    let mut prev = 0;
    let mut cur = 1;
    let mut total = 0;
    while cur < hi {
        if cur % 2 == 0 {
            total += cur;
        }
        let tmp = cur;
        cur += prev;
        prev = tmp;
    }
    total
}
