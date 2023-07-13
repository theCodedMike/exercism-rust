use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut result = nucleotide_counts(dna)?;
    result.remove(&nucleotide).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = ['A', 'C', 'G', 'T']
        .into_iter()
        .map(|c| (c, 0))
        .collect::<HashMap<char, usize>>();

    for c in dna.chars() {
        map.get_mut(&c).map(|v| *v += 1).ok_or(c)?
    }

    Ok(map)
}
