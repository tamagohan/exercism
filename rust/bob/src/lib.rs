use std::collections::HashSet;
pub fn reply(message: &str) -> &str {
    let spaces: HashSet<char> = " \t\r".chars().collect();
    let uniq: HashSet<char> = message.chars().collect();
    if uniq.is_subset(&spaces) {
        return "Fine. Be that way!";
    }

    let is_question = message.trim_end_matches(" ").ends_with("?");
    let is_uppercase_or_symbol = message.to_uppercase() == message;
    let is_contains_alphabetic = !message
        .matches(char::is_alphabetic)
        .collect::<Vec<&str>>()
        .is_empty();

    if is_question {
        if is_contains_alphabetic && is_uppercase_or_symbol {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Sure.";
        }
    }

    if is_contains_alphabetic && is_uppercase_or_symbol {
        return "Whoa, chill out!";
    }

    return "Whatever.";
}
