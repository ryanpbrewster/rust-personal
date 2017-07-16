/*
The most naive way of computing n15 requires fourteen multiplications:

n × n × ... × n = n^15

But using a "binary" method you can compute it in six multiplications:

n × n = n^2
n^2 × n^2 = n^4
n^4 × n^4 = n^8
n^8 × n^4 = n^12
n^12 × n^2 = n^14
n^14 × n = n^15

However it is yet possible to compute it in only five multiplications:

n × n = n^2
n^2 × n = n^3
n^3 × n^3 = n^6
n^6 × n^6 = n^12
n^12 × n^3 = n^15

We shall define m(k) to be the minimum number of multiplications to compute nk;
for example m(15) = 5.

For 1 ≤ k ≤ 200, find ∑ m(k).

N.B.: I think the general version of this problem is really hard, but the
numbers here are low enough that we can safely assume that the most efficient
multiplication scheme always uses the previously generated value.  Look up
"star chains", which are a special case of "addition chains"
*/

use std::collections::VecDeque;

type AdditionChain = Vec<usize>;
type ChainLength = usize;

pub fn solve(bound: usize) -> Vec<ChainLength> {
    let mut chain_lengths: Vec<ChainLength> = vec![0; bound + 1];
    let mut num_unsolved = bound;

    let mut q: VecDeque<AdditionChain> = VecDeque::new();
    q.push_back(vec![1]);

    // Invariant: !chain.is_empty() && chain.last() <= bound
    while let Some(chain) = q.pop_front() {
        let len = chain.len();
        let last = chain[len - 1];
        if chain_lengths[last] == 0 {
            chain_lengths[last] = len;
            num_unsolved -= 1;
            println!("l({}) = {}, {} to go", last, len, num_unsolved);
            if num_unsolved == 0 {
                break;
            }
        }

        for &v in chain.iter().rev() {
            if v + last <= bound && chain_lengths[v + last] == 0 {
                let mut child = chain.clone();
                child.push(v + last);
                q.push_back(child);
            }
        }
    }

    chain_lengths
}

#[cfg(test)]
mod test {
    use super::*;

    fn chain_length_sum(bound: usize) -> usize {
        solve(bound)[1..].into_iter().sum::<usize>() - bound
    }

    #[test]
    fn small() {
        let chain_lengths = solve(15);
        assert_eq!(chain_lengths[1], 1);
        assert_eq!(chain_lengths[2], 2);
        assert_eq!(chain_lengths[3], 3);
        assert_eq!(chain_lengths[4], 3);
        assert_eq!(chain_lengths[15], 6);
    }

    #[test]
    fn main() {
        assert_eq!(chain_length_sum(200), 1582);
    }

    #[test]
    fn big() {
        assert_eq!(chain_length_sum(2_000), 24063);
    }
}

