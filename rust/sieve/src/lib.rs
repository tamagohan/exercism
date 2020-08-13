fn is_prime(n: u64) -> bool {
    !(2..(n / 2 + 1)).any(|i| n % i == 0)
}

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    (2..(upper_bound + 1)).filter(|&i| is_prime(i)).collect()
}
