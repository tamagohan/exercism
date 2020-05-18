pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let factors_without_zero = factors.iter().filter(|x| *x != &0).collect::<Vec<&u32>>();
    (1..limit)
        .filter(|x| factors_without_zero.iter().any(|f| x % *f == 0))
        .sum()
}
