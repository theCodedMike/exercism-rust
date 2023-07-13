use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !"ACGT".contains(nucleotide) {
        return Err(nucleotide);
    }

    let mut count = 0;
    for c in dna.chars() {
        match c {
            'A' | 'C' | 'G' | 'T' => {
                if nucleotide == c {
                    count += 1;
                }
            }
            _ => return Err(c),
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::with_capacity(4);
    map.insert('A', 0);
    map.insert('C', 0);
    map.insert('G', 0);
    map.insert('T', 0);

    for c in dna.chars() {
        match c {
            'A' | 'C' | 'G' | 'T' => {
                map.entry(c).and_modify(|v| *v += 1).or_insert(0);
            }
            _ => return Err(c),
        }
    }

    Ok(map)
}
