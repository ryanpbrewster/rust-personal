pub fn sieve(hi: usize) -> Vec<usize> {
    let mut is_prime_vec : Vec<bool> = vec![true; hi];
    for p in 2..hi {
        if is_prime_vec[p] {
            for k in p..hi/p {
                is_prime_vec[k*p] = false;
            }
        }
    }
    (2usize..hi).filter(|&p| is_prime_vec[p]).collect()
}
