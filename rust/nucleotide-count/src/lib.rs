use std::collections::HashMap;

static NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

fn validate(nucleotide: char) -> Result<char, char> {
    NUCLEOTIDES
        .iter()
        .cloned()
        .find(|c| *c == nucleotide)
        .ok_or(nucleotide)
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
