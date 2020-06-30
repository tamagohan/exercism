use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match valid(nucleotide) {
        Err(c) => Err(c),
        Ok(_) => dna
            .chars()
            .map(|c| valid(c))
            .collect::<Result<Vec<char>, char>>()
            .map(|v| v.into_iter().filter(|c| *c == nucleotide).count()),
    }
}

fn valid(nucleotide: char) -> Result<char, char> {
    match nucleotide {
        c if NUCLEOTIDES.contains(&c) => Ok(nucleotide),
        _ => Err(nucleotide),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    NUCLEOTIDES
        .iter()
        .map(|n| count(*n, dna).map(|count| (*n, count)))
        .collect()
}
