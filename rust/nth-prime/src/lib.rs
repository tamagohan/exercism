pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(|i| is_prime_number(*i))
        .nth(n as usize)
        .unwrap()
}

fn is_prime_number(n: u32) -> bool {
    !(2..(n / 2 + 1)).any(|i| n % i == 0)
}
