use itertools::Itertools;

pub fn encode_char(c: char) -> char {
    if c.is_numeric() {
        c
    } else {
        let pos_from_a = c.to_ascii_lowercase() as u8 - ('a' as u8);
        (('z' as u8) - pos_from_a) as char
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(encode_char)
        .collect::<Vec<_>>()
        .chunks(5)
        .intersperse(&[' '])
        .flatten()
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(encode_char)
        .collect()
}
