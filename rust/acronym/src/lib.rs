enum LastChar {
    Nothing,
    Lowercase,
    Uppercase,
    Separator,
}

pub fn abbreviate(phrase: &str) -> String {
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
