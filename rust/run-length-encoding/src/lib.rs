extern crate itertools;
extern crate regex;

use regex::Regex;

use itertools::Itertools;

/// Run-length encode a string. String must only consist of
/// a-z, A-Z, and whitespace
pub fn encode(source: &str) -> String {
    source
        .chars()
        .group_by(|&c| c)
        .into_iter()
        .map(|(ch, run)| match run.count() {
            1 => ch.to_string(),
            len => len.to_string() + ch.to_string().as_str(),
        }).collect()
}

pub fn decode(source: &str) -> String {
    if source.len() == 0 {
        return String::new();
    }

    let char_regex = Regex::new(r"[a-zA-Z ]").unwrap();
    let run_lens = source
        .split(|ch: char| char_regex.is_match(ch.to_string().as_str()))
        .map(|x| x.parse::<usize>());
    let char_seq = source
        .chars()
        .filter(|ch| char_regex.is_match(ch.to_string().as_str()));

    char_seq
        .zip(run_lens)
        .map(|(ch, len)| ch.to_string().repeat(len.unwrap_or(1)))
        .collect()
}
