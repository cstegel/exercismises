use std::collections::HashMap;

use itertools::Itertools;

/// Count occurrences of words.
/// A word is identified as an adjacent grouping of alphanumeric characters and apostraphes
/// in their lower-case form except apostraphes cannot be the first or last char in the word
pub fn word_count(sentence: &str) -> HashMap<String, u32> {
    sentence
        .to_lowercase()
        .chars()
        .group_by(|c| c.is_alphanumeric() || *c == '\'')
        .into_iter()
        .filter(|(is_word, _)| *is_word)
        .map(|(_, char_iter)| char_iter.collect::<String>().trim_matches('\'').to_string())
        .fold(HashMap::new(), |mut counts, word| {
            *counts.entry(word).or_insert(0) += 1;
            counts
        })
}
