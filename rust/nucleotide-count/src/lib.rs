use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match valid(nucleotide) {
        Err(c) => Err(c),
        Ok(_) => dna
            .chars()
            .map(|c| valid(c).and_then(|c| if c == nucleotide { Ok(1) } else { Ok(0) }))
            .sum::<Result<usize, char>>(),
    }
}

fn valid(nucleotide: char) -> Result<char, char> {
    let nucleotides = ['A', 'C', 'G', 'T'];
    match nucleotide {
        c if nucleotides.contains(&c) => Ok(nucleotide),
        _ => Err(nucleotide),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    unimplemented!(
        "How much of every nucleotide type is contained inside DNA string '{}'?",
        dna
    );
}
