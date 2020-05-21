use std::convert::TryInto;

pub fn square(s: u32) -> u64 {
    if s >= 1 && s <= 64 {
        2_u64.pow(s - 1)
    } else {
        panic!("Square must be between 1 and 64")
    }
}

pub fn total() -> u64 {
    let total_number_of_squares: u32 = 64;
    // Use Geometric Series Sum Formula
    (2_u128.pow(total_number_of_squares) - 1)
        .try_into()
        .unwrap()
}
