const SONGS: [&str; 3] = [
    "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
    "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n",
    "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n",
];

pub fn verse(n: u32) -> String {
    match n {
        n if n < 3 => SONGS[n as usize].to_string(),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1).to_string(),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..(start + 1))
        .map(|n| verse(n))
        .rev()
        .collect::<Vec<String>>()
        .join("\n")
}
