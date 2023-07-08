use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(score, letters)| {
            letters
                .iter()
                .map(|l| (((*l as u8) + 32) as char, *score))
                .collect::<BTreeMap<char, i32>>()
        })
        .collect()
}
