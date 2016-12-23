use std::collections::HashSet;

pub fn ways_to_make(target: usize, coins: &Vec<usize>) -> u32 {
    let mut ways_arr = vec![0; target+1];
    ways_arr[0] = 1;
    for &c in coins {
        for i in (c..(target+1)).rev() {
            ways_arr[i] += ways_arr[i-c];
        }
    }
    ways_arr[target]
}

pub fn ways_to_make_with_replacement(target: usize, coins: &HashSet<usize>) -> u32 {
    let mut coins_vec: Vec<usize> = coins.iter().map(|v| v.clone()).collect::<Vec<_>>();
    coins_vec.sort();
    let sorted_coins = coins_vec;

    let mut ways_arr = vec![0; target+1];
    ways_arr[0] = 1;
    for c in sorted_coins {
        for i in c..(target+1) {
            ways_arr[i] += ways_arr[i-c];
        }
    }
    ways_arr[target]
}
