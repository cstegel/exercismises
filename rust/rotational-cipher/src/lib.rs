pub fn encode_char(c: char, key: i8) -> char {
    if c.is_ascii_alphabetic() {
        let start_char = if c.is_ascii_lowercase() { 'a' } else { 'A' };

        let pos_from_start = (c as u8) - (start_char as u8);
        ((start_char as u8) + ((pos_from_start as i16 + key as i16) % 26) as u8) as char
    } else {
        c
    }
}

pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|c| encode_char(c, key)).collect()
}
