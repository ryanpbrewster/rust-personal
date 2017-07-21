use num::{BigUint, Integer, ToPrimitive};
use num::{One, Zero};

pub fn solve(num_turns: usize) -> u32 {
    let mut g = GameState::new();
    for _ in 0..num_turns {
        g.play_one();
    }

    let total: BigUint = g.blue_count_ways.iter().fold(
        BigUint::zero(),
        |acc, x| acc + x,
    );

    let blues_to_win = num_turns / 2 + 1;
    let winning: BigUint = g.blue_count_ways[blues_to_win..].iter().fold(
        BigUint::zero(),
        |acc, x| acc + x,
    );

    total.div_floor(&winning).to_u32().expect(
        "i hope that this fits into a u32",
    )
}

struct GameState {
    turn: usize,
    blue_discs: u32,
    red_discs: u32,
    blue_count_ways: Vec<BigUint>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            turn: 0,
            blue_discs: 1,
            red_discs: 1,
            blue_count_ways: vec![BigUint::one()],
        }
    }

    pub fn play_one(&mut self) {
        let prev = self.blue_count_ways.clone();

        let ways_0 = &prev[0] * BigUint::from(self.red_discs); // picking any of the red discs
        self.blue_count_ways[0] = ways_0;

        let ways_n = &prev[self.turn] * BigUint::from(self.blue_discs); // picking any blue disc
        self.blue_count_ways.push(ways_n);

        for i in 1..prev.len() {
            // To get `i` blue discs we can either:
            //   - start with `i-1` and pick blue
            //   - start with `i` and pick red
            let ways_i = &prev[i - 1] * BigUint::from(self.blue_discs) +
                &prev[i] * BigUint::from(self.red_discs);
            self.blue_count_ways[i] = ways_i;
        }

        self.turn += 1;
        self.red_discs += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        assert_eq!(solve(4), 10);
    }

    #[test]
    fn main() {
        assert_eq!(solve(15), 2269);
    }
}
