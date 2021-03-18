pub fn reply(message: &str) -> &str {
    let message_without_whitespaces: String = message.split_whitespace().collect();

    let all_alphabetic_chars = message_without_whitespaces
        .chars()
        .filter(|x| x.is_alphabetic())
        .collect::<Vec<char>>();

    if !all_alphabetic_chars.is_empty()
        && all_alphabetic_chars.iter().all(|x| x.is_uppercase())
        && message_without_whitespaces.chars().last().unwrap() == '?'
    {
        return "Calm down, I know what I'm doing!";
    }

    if !message_without_whitespaces.is_empty()
        && message_without_whitespaces.chars().last().unwrap() == '?'
    {
        return "Sure.";
    }

    if !all_alphabetic_chars.is_empty() && all_alphabetic_chars.iter().all(|x| x.is_uppercase()) {
        return "Whoa, chill out!";
    }

    if message_without_whitespaces.is_empty() {
        return "Fine. Be that way!";
    }

    "Whatever."
}
