pub fn reverse(input: &str) -> String {
    let mut v: Vec<char> = input.chars().collect();
    v.reverse();
    v.iter().collect()
}
