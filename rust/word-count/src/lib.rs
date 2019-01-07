use std::collections::HashMap;

/// Count occurrences of words.
/// A word is identified as an adjacent grouping of alphanumeric characters and apostraphes
/// in their lower-case form except apostraphes cannot be the first or last char in the word
pub fn word_count(sentence: &str) -> HashMap<String, u32> {
    sentence
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .map(|substr| substr.trim_matches('\''))
        .filter(|substr| !substr.is_empty())
        .fold(HashMap::new(), |mut counts, word| {
            *counts.entry(word.to_string()).or_insert(0) += 1;
            counts
        })
}
