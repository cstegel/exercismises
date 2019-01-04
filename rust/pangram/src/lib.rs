use itertools::Itertools;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic() && c.is_ascii())
        .unique()
        .count()
        == 26
}
