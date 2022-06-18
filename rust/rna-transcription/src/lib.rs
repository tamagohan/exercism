#[derive(Debug, PartialEq)]
pub struct DNA {
    strand: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    strand: String,
}

impl DNA {
    const ADENINE: char = 'A';
    const CYTOSINE: char = 'C';
    const GUANINE: char = 'G';
    const THYMINE: char = 'T';

    pub fn new(dna: &str) -> Result<DNA, usize> {
        let invalid_char_index = dna.find(|c| {
            c != Self::GUANINE && c != Self::CYTOSINE && c != Self::THYMINE && c != Self::ADENINE
        });
        match invalid_char_index {
            Some(n) => Err(n),
            None => Ok(DNA {
                strand: dna.to_string(),
            }),
        }
    }

    pub fn into_rna(self) -> RNA {
        let rna_strand = self
            .strand
            .replace(Self::ADENINE, &RNA::URACIL.to_string())
            .replace(Self::THYMINE, &RNA::ADENINE.to_string())
            .replace(Self::GUANINE, "X")
            .replace(Self::CYTOSINE, &RNA::GUANINE.to_string())
            .replace("X", &RNA::CYTOSINE.to_string());
        RNA { strand: rna_strand }
    }
}

impl RNA {
    const ADENINE: char = 'A';
    const CYTOSINE: char = 'C';
    const GUANINE: char = 'G';
    const URACIL: char = 'U';

    pub fn new(rna: &str) -> Result<RNA, usize> {
        let invalid_char_index = rna.find(|c| {
            c != Self::ADENINE && c != Self::CYTOSINE && c != Self::GUANINE && c != Self::URACIL
        });
        match invalid_char_index {
            Some(n) => Err(n),
            None => Ok(RNA {
                strand: rna.to_string(),
            }),
        }
    }
}
