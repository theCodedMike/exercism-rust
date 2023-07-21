#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    nucleotides: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    nucleotides: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let nucleotides = process_xna(dna, true)?;
        Ok(Dna { nucleotides })
    }

    pub fn into_rna(self) -> Rna {
        Rna {
            nucleotides: self
                .nucleotides
                .chars()
                .map(|c| match c {
                    'A' => 'U',
                    'T' => 'A',
                    'G' => 'C',
                    _ => 'G',
                })
                .collect(),
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let nucleotides = process_xna(rna, false)?;
        Ok(Rna { nucleotides })
    }
}

fn process_xna(xna: &str, is_dna: bool) -> Result<String, usize> {
    let mut nucleotides = "".to_string();

    for (idx, c) in xna.chars().enumerate() {
        match c {
            'A' | 'G' | 'C' => nucleotides.push(c),
            'T' => {
                if is_dna {
                    nucleotides.push(c);
                } else {
                    return Err(idx);
                }
            }
            'U' => {
                if is_dna {
                    return Err(idx);
                } else {
                    nucleotides.push(c);
                }
            }
            _ => return Err(idx),
        }
    }

    Ok(nucleotides)
}
