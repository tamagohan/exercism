pub fn verse(n: u32) -> String {
    match n {
        0 => format!("{} bottles of beer on the wall, {} bottles of beer.\nGo to the store and buy some more, {} bottles of beer on the wall.\n", "No more", "no more", 99).to_string(),
        1 => format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, {} bottles of beer on the wall.\n", 1, 1, "no more").to_string(),
        2 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n - 1).to_string(),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1).to_string(),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..(start + 1))
        .map(|n| verse(n))
        .rev()
        .collect::<Vec<String>>()
        .join("\n")
        .to_string()
}
