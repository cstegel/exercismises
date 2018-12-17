extern crate regex;

use regex::Regex;

/// Run-length encode a string. String must only consist of
/// a-z, A-Z, and whitespace
pub fn encode(source: &str) -> String {
    println!("hi");
    if source.len() == 0 {
        println!("empty");
        return String::new();
    }

    let mut run_seq: Vec<(char, usize)> = Vec::new();
    let mut cur_char = source.chars().next().unwrap();
    let mut run_len: usize = 0;
    for ch in source.chars() {
        if ch != cur_char {
            run_seq.push((cur_char, run_len));
            cur_char = ch;
            run_len = 0;
        }
        run_len += 1;
    }
    println!("end, {}, {}", cur_char, run_len);
    run_seq.push((cur_char, run_len));

    run_seq
        .iter()
        .map(|(ch, len)| match len {
            1 => ch.to_string(),
            _ => ch.to_string().repeat(*len),
        }).collect()
}

pub fn decode(source: &str) -> String {
    if source.len() == 0 {
        return String::new();
    }

    let char_regex = Regex::new(r"[a-zA-Z ]").unwrap();

    let run_lens = source
        .split(|ch: char| char_regex.is_match(ch.to_string().as_str()))
        .filter(|s| s.len() > 0)
        .map(|x| {
            println!("'{}'", x);
            x.parse::<usize>()
                .expect(format!("Invalid run length, must be number: {}", x).as_str())
        });
    let char_seq = source.chars().filter(|ch| ch.is_alphabetic() || *ch == ' ');

    run_lens
        .zip(char_seq)
        .map(|(len, ch)| ch.to_string().repeat(len))
        .collect()
}
