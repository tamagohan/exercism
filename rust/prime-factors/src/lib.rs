use std::vec::Vec;

fn factors_imp(n: u64, l: u64, list: &mut Vec<u64>) {
    if n % l == 0 {
        list.push(l);
        factors_imp(n / l, l, list);
    } else {
        if l < n {
            factors_imp(n, l + 1, list);
        } else {
            return;
        }
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    factors_imp(n, 2, &mut factors);
    factors
}
