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

    let to_acro_char = |ch: char| ch.to_uppercase().collect::<String>();

    for ch in phrase.chars() {
        prev_char = match prev_char {
            LastChar::Nothing => {
                if ch.is_alphabetic() {
                    acronym.push_str(to_acro_char(ch).as_str());
                    alphabetic_state(ch)
                } else {
                    LastChar::Separator
                }
            }
            LastChar::Lowercase => {
                if ch.is_alphabetic() {
                    if ch.is_uppercase() {
                        acronym.push_str(to_acro_char(ch).as_str());
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
                    acronym.push_str(to_acro_char(ch).as_str());
                    alphabetic_state(ch)
                } else {
                    LastChar::Separator
                }
            }
        }
    }
    acronym
}
