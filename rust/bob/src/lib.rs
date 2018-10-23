///
/// answers 'Sure.' if you ask him a question.
/// answers 'Whoa, chill out!' if you yell at him.
/// answers 'Calm down, I know what I'm doing!' if you yell a question at him.
/// answers 'Fine. Be that way!' if you address him without actually saying anything.
/// answers 'Whatever.' to anything else.
pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = trimmed.chars().last().unwrap() == '?';
    let mut alpha_chars = trimmed.chars().filter(|c| c.is_alphabetic()).peekable();

    let is_yelling = alpha_chars.peek().is_some() && alpha_chars.all(|c| c.is_uppercase());

    match (is_question, is_yelling) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
}
