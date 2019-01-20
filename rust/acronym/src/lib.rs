enum LastChar {
    Nothing,
    Lowercase,
    Uppercase,
    Separator,
}

pub fn abbreviate_state_machine(phrase: &str) -> String {
    let mut prev_char = LastChar::Nothing;

    let mut acronym = String::new();
    let alphabetic_state = |ch: char| {
        if ch.is_uppercase() {
            LastChar::Uppercase
        } else {
            LastChar::Lowercase
        }
    };

    for ch in phrase.chars() {
        prev_char = match prev_char {
            LastChar::Nothing => {
                if ch.is_alphabetic() {
                    acronym.push(ch);
                    alphabetic_state(ch)
                } else {
                    LastChar::Separator
                }
            }
            LastChar::Lowercase => {
                if ch.is_alphabetic() {
                    if ch.is_uppercase() {
                        acronym.push(ch);
                    }
                    alphabetic_state(ch)
                } else {
                    LastChar::Separator
                }
            }
            LastChar::Uppercase => {
                if ch.is_alphabetic() {
                    alphabetic_state(ch)
                } else {
                    LastChar::Separator
                }
            }
            LastChar::Separator => {
                if ch.is_alphabetic() {
                    acronym.push(ch);
                    alphabetic_state(ch)
                } else {
                    LastChar::Separator
                }
            }
        }
    }
    acronym.to_uppercase()
}

///
/// Better than abbreviate_state_machine due to less code
/// while still maintaining a single pass through the string
/// State machine isn't necessary since only the immediately previous character
/// is needed to be known
pub fn abbreviate_fold(phrase: &str) -> String {
    let (acronym, _) = phrase
        .chars()
        .fold((String::new(), ' '), |(mut acronym, prev_char), ch| {
            if !prev_char.is_alphabetic() && ch.is_alphabetic() {
                acronym.push(ch);
            } else if prev_char.is_lowercase() && ch.is_uppercase() {
                acronym.push(ch);
            }
            (acronym, ch)
        });
    acronym.to_uppercase()
}

pub fn abbreviate(phrase: &str) -> String {
    abbreviate_fold(phrase)
}
