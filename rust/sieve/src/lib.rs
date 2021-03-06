pub fn primes_up_to(upper_bound: usize) -> Vec<u64> {
    let u: usize = upper_bound as usize;
    let mut primes = vec![false; u + 1];

    (2..=u)
        .filter_map(|i| {
            if primes[i] {
                return None;
            }

            for m in 1..=(u / i) {
                primes[m * i] = true
            }
            Some(i as u64)
        })
        .collect()
}
