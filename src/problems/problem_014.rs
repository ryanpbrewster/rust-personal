use std::ops::Range;

// Find the number in the `seeds` Range that produces the longest Collatz chain
pub fn solve(seeds: Range<u64>) -> u64 {
    seeds.max_by_key(|&s| Collatz::start(s).count()).expect("`seeds` must not be empty")
}

struct Collatz(u64);

impl Collatz {
    pub fn start(seed: u64) -> Collatz {
        Collatz(seed)
    }
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 1 {
            return None;
        }

        let v = self.0;
        if self.0 % 2 == 0 {
            self.0 = self.0 / 2;
        } else {
            self.0 = 3 * self.0 + 1;
        }
        Some(v)
    }
}


// A faster "clever" (maybe ironic, it's unclear to me) solution
pub fn solve_fast(seeds: Range<usize>) -> usize {
    let mut memo: Vec<Option<usize>> = vec![None; seeds.end];
    memo[1] = Some(1);

    // The recursion gets deep enough that we run into stack overflows, so we'll manually
    // manage the stack. :sigh:
    let mut stack: Vec<usize> = Vec::new();
    
    seeds.max_by_key(|&s| {
        let mut cur = s;
        while cur >= memo.len() || memo[cur].is_none() {
            stack.push(cur);
            cur = if cur % 2 == 0 {
                cur / 2
            } else {
                3 * cur + 1
            }
        }

        let mut len = memo[cur].unwrap();
        while let Some(cur) = stack.pop() {
            len += 1;
            if cur < memo.len() {
                memo[cur] = Some(len);
            }
        }
        len
    }).unwrap()
}
