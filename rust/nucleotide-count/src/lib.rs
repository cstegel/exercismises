use std::collections::HashMap;
use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref NUCLEOTIDES: HashSet<char> = "ACGT".chars().collect();
}

fn validate(candidate: char) -> Result<char, char> {
    if NUCLEOTIDES.contains(&candidate) {
        Ok(candidate)
    } else {
        Err(candidate)
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    validate(nucleotide)?;
    dna.chars().map(validate).collect::<Result<Vec<_>, _>>()?;
    Ok(dna.chars().filter(|c| *c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    NUCLEOTIDES
        .iter()
        .map(|nuc| count(*nuc, dna).map(|size| (*nuc, size)))
        .collect()
}
