extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // let rev_graphemes: Vec<_> = input.graphemes(true).rev().collect();
    // return rev_graphemes.join("");

    // flat_map() returns a single iterator for its closure's return type, Chars
    // which String implements FromIter<Chars> so collect() is compatible.
    // If map() was used then the return type would be some Iterator<Iterator<Chars>>
    // which String does not implement FromIter for
    input
        .graphemes(true)
        .rev()
        .flat_map(|s| s.chars())
        .collect()
    // input.chars().rev().collect()
}
