pub fn abbreviate(phrase: &str) -> String {
    let words = phrase.split(|c| c == ' ' || c == '-').collect::<Vec<_>>();
    let first_word = words[0];
    if first_word.ends_with(":") {
        return first_word.to_string().trim_end_matches(':').to_string();
    }
    words
        .iter()
        .flat_map(|word| {
            let trimmed_word = word.trim_matches(|c: char| !c.is_alphabetic());
            trimmed_word.chars().take(1).chain(
                trimmed_word
                    .chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .map(|c| c.to_ascii_uppercase())
        .collect()
}
