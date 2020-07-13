pub fn nth(n: u32) -> u32 {
    (0..n).fold(2, |acc, _| next_prime(acc))
}

fn next_prime(min: u32) -> u32 {
    let mut n = min + 1;
    while !is_prime_number(n) {
        n = n + 1;
    }
    n
}

fn is_prime_number(n: u32) -> bool {
    if n < 3 {
        true
    } else {
        (2..(n / 2 + 1)).all(|num| n % num != 0)
    }
}
