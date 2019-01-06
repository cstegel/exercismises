use std::collections::HashMap;

use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref valid_nucleotides: HashSet<char> = "ACGT".chars().collect();
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !valid_nucleotides.contains(&nucleotide) {
        return Err(nucleotide);
    }

    // check that all nucleotides are valid
    if let Some(invalid) = dna.chars().find(|c| !valid_nucleotides.contains(c)) {
        return Err(invalid);
    }

    Ok(dna.chars().filter(|c| *c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for nucleotide in valid_nucleotides.iter() {
        counts.insert(*nucleotide, count(*nucleotide, dna)?);
    }
    Ok(counts)
}
