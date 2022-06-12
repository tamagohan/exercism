#[derive(Debug, PartialEq)]
pub struct DNA {
    strand: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    strand: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let invalid_char_index = dna.find(|c| c != 'G' && c != 'C' && c != 'T' && c != 'A');
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
            .replace("A", "U")
            .replace("T", "A")
            .replace("G", "X")
            .replace("C", "G")
            .replace("X", "C");
        RNA { strand: rna_strand }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let invalid_char_index = rna.find(|c| c != 'A' && c != 'C' && c != 'G' && c != 'U');
        match invalid_char_index {
            Some(n) => Err(n),
            None => Ok(RNA {
                strand: rna.to_string(),
            }),
        }
    }
}
