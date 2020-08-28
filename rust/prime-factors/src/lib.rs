use std::vec::Vec;

fn cal_factors(n: u64, l: u64, list: &mut Vec<u64>) {
    if n % l == 0 {
        list.push(l);
        cal_factors(n / l, l, list);
    } else {
        if l < n {
            cal_factors(n, l + 1, list);
        } else {
            return;
        }
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    cal_factors(n, 2, &mut factors);
    factors
}
