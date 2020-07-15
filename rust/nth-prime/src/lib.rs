pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(|x| is_prime_number(*x))
        .nth(n as usize)
        .unwrap()
}

fn is_prime_number(n: u32) -> bool {
    !(2..(n / 2 + 1)).any(|num| n % num == 0)
}
