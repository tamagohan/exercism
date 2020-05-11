fn is_yell(message: &str) -> bool {
    let is_uppercase_or_symbol = message.to_uppercase() == message;
    let is_contains_alphabetic = !message
        .matches(char::is_alphabetic)
        .collect::<Vec<&str>>()
        .is_empty();

    is_uppercase_or_symbol && is_contains_alphabetic
}
pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.len() == 0 => "Fine. Be that way!",
        m if m.ends_with("?") && is_yell(m) => "Calm down, I know what I'm doing!",
        m if m.ends_with("?") => "Sure.",
        m if is_yell(m) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
