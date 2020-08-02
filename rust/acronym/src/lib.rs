fn to_abbreviate(word: &str) -> String {
    if !word.is_empty() && word.chars().all(|c| c.is_uppercase()) {
        return word
            .chars()
            .next()
            .unwrap()
            .to_ascii_uppercase()
            .to_string();
    }
    word.chars()
        .filter(|c| c.is_uppercase())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(" ")
        .map(|word| {
            println!("{:?}", word);
            to_abbreviate(word)
        })
        .collect()
}
