fn to_abbreviate(word: &str) -> String {
    if word.is_empty() {
        return word.to_string();
    }

    let first_letter = word.chars().next().unwrap();
    if first_letter.is_lowercase() {
        return first_letter.to_ascii_uppercase().to_string();
    } else if word.chars().all(|c| c.is_uppercase()) {
        return first_letter.to_ascii_uppercase().to_string();
    } else {
        word.chars()
            .filter(|c| c.is_uppercase())
            .map(|c| c.to_ascii_uppercase())
            .collect()
    }
}

pub fn abbreviate(phrase: &str) -> String {
    let words = phrase.split(|c| c == ' ' || c == '-').collect::<Vec<_>>();
    let first_word = words[0];
    if first_word.ends_with(":") {
        return first_word.to_string().trim_end_matches(':').to_string();
    }

    words
        .iter()
        .map(|word| {
            println!("{:?}", word);
            to_abbreviate(word)
        })
        .collect()
}
