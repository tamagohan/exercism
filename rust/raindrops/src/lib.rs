pub fn raindrops(n: u32) -> String {
    let is_factor = |factor: u32| n % factor == 0;
    let mut result = String::new();
    if is_factor(3) {
        result.push_str("Pling");
    }
    if is_factor(5) {
        result.push_str("Plang");
    }
    if is_factor(7) {
        result.push_str("Plong");
    }

    if result.is_empty() {
        result = n.to_string();
    }

    return result;
}
